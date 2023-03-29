//! This module provides the [`InterCom`] struct which is used by the [`Communicator`] to send messages to the specified [`handler`](Communicator::handler).


use std::sync::Arc;

use common::{
    communicator::message::Message,
    config,
    error,
    info,
    mcmanage_error::MCManageError,
    status::Status,
    types::ThreadJoinHandle
};
use proc_macros::ConcurrentClass;
use tokio::{
    spawn,
    sync::{
        Mutex,
        mpsc::{
            channel,
            Sender,
            Receiver
        },
        oneshot
    },
    time::Instant,
};

use super::Communicator;


mod tests;


/// This struct is used by the [`Communicator`] to send messages to the specified [`handler`](Communicator::handler).
#[derive(ConcurrentClass)]
pub struct InterCom {
    /// This struct's name
    name: String,
    /// The main thread of this struct
    main_thread: Arc<Mutex<Option<ThreadJoinHandle>>>,
    /// The [`Status`] of this struct
    status: Mutex<Status>,

    /// This is the [`Communicator`] of this [`InterCom`]
    communicator: Mutex<Option<Arc<Communicator>>>,
    /// This is a receiving channel for receiving messages from the [`Communicator`]'s [`send_message`](common::communicator::CommunicatorTrait::send_message)
    receiver: Mutex<Receiver<Message>>,
    /// This is a list of all sender channels to the [`Communicator`]'s handlers
    handler_send: Mutex<Vec<Option<Sender<Message>>>>,
    /// This is a list of all [`recv_handler`](InterCom::recv_handler)s
    handlers: Mutex<Vec<Option<ThreadJoinHandle>>>
}
impl InterCom {
    /// Create a new [`InterCom`] instance. \
    /// \
    /// Note: The returned struct will remain non-functional until a [`Communicator`] got set via the [`set_communicator`](Self::set_communicator) method.
    pub async fn new(receiver: Receiver<Message>) -> Arc<Self> {
        Arc::new(Self {
            name: "InterCom".into(),
            main_thread: Arc::new(None.into()),
            status: Mutex::new(Status::Stopped),

            communicator: None.into(),
            receiver: receiver.into(),
            handler_send: vec![].into(),
            handlers: vec![].into()
        })
    }
    /// Set the [`Communicator`] owning this [`InterCom`]. \
    /// The [`InterCom`] only becomes functional if a [`Communicator`] got set.
    pub async fn set_communicator(self: &Arc<Self>, communicator: &Arc<Communicator>) {
        *self.communicator.lock().await = Some(communicator.clone());
    }

    /// This is the blocking implementation to start a given struct. \
    /// For a non-blocking mode use the [`start method`](Self::start). \
    /// \
    /// The `restart` parameter will be used by the [`restart method`](Self::impl_restart) to deactivate all non-fatal error messages of this method and to enable
    /// this method to be executed during a restart.
    pub async fn impl_start(self: Arc<Self>, restart: bool) -> Result<(), MCManageError> {
        self.check_allowed_start(restart).await?;

        if (self.communicator.lock().await).is_none() {
            error!(self.name, "The Communicator has not yet been set.");
            return Err(MCManageError::NotReady);
        }

        if !restart { info!(self.name, "Starting..."); }
        let start_time = Instant::now();

        let rx = self.start_main_thread().await;
        self.recv_start_result(rx, restart).await;
        *self.status.lock().await = Status::Started;

        if !restart { info!(self.name, "Started in {:.3} secs!", start_time.elapsed().as_secs_f64()); }
        Ok(())
    }
    /// This is the blocking implementation to stop a given struct. \
    /// For a non-blocking mode use the [`stop method`](Self::stop). \
    /// \
    /// The `restart` parameter will be used by the [`restart method`](Self::impl_restart) to deactivate all non-fatal error messages of this method and to enable
    /// this method to be executed during a restart. \
    /// \
    /// The `forced` parameter is used to wait for a given struct to start / stop to ensure a stop attempt.
    pub async fn impl_stop(self: Arc<Self>, restart: bool, forced: bool) -> Result<(), MCManageError> {
        self.check_allowed_stop(restart, forced).await?;

        if !restart { info!(self.name, "Shutting down..."); }
        let stop_time = Instant::now();

        self.stop_main_thread().await;
        for slot in &mut *self.handlers.lock().await {
            if let Some(handler) = slot {
                handler.abort();
                *slot = None;
            }
        }
        *self.status.lock().await = Status::Stopped;

        if !restart { info!(self.name, "Stopped in {:.3} secs!", stop_time.elapsed().as_secs_f64()); }
        Ok(())
    }
    /// Reset a given struct to its starting values.
    async fn reset(self: &Arc<Self>) {
        if let Some(thread) = self.main_thread.lock().await.take() { thread.abort(); }
        *self.status.lock().await = Status::Stopped;
        *self.handler_send.lock().await = vec![];
        for slot in &mut *self.handlers.lock().await {
            if let Some(handler) = slot {
                handler.abort();
                *slot = None;
            }
        }
        *self.handlers.lock().await = vec![];
    }

    /// This method adds an element `T` to a vector of type `Option<T>` at the specified position. \
    /// \
    /// If the given vector is too short, a None will be appended until the element can be appended. \
    /// Note: This method will override existing data at the given position inside the vector.
    async fn add_to_option_list<T>(self: &Arc<Self>, option_list: &mut Vec<Option<T>>, element: T, position: usize) {
        let mut i = 0;
        for slot in &mut *option_list {
            if position == i {
                *slot = Some(element);
                return;
            }
            i+=1;
        }
        loop {
            if i == position {
                option_list.push(Some(element));
                return;
            } else {
                option_list.push(None);
            }
            i+=1;
        }
    }
    /// As the name suggests, this method will add a [`handler`](Communicator::handler) to the [`InterCom`]. \
    /// In other words, this method will
    ///     1. Start a [`handler`](Self::recv_handler) task.
    ///     2. Add the channels used to communicate with the [`Communicator's`](Communicator) [`handler`](Communicator::handler).
    pub async fn add_handler(self: &Arc<Self>, handler_id: u64) -> Result<(Sender<Message>, Receiver<Message>), MCManageError> {
        if (self.communicator.lock().await).is_none() {
            error!(self.name, "The Communicator has not yet been set.");
            return Err(MCManageError::NotReady);
        }

        let (tx, handler_recv) = channel(config::buffsize().await);
        let (handler_send, rx) = channel(config::buffsize().await);

        self.add_to_option_list(
            &mut *self.handler_send.lock().await,
            tx,
            handler_id as usize
        ).await;
        self.add_to_option_list(
            &mut *self.handlers.lock().await,
            spawn(self.clone().recv_handler(rx)),
            handler_id as usize
        ).await;

        Ok((handler_send, handler_recv))
    }
    /// As the name suggests, this method will remove a [`handler`](Communicator::handler) from the [`InterCom`]. \
    /// In other words, this method will
    ///     1. Abort the [`handler`](Self::recv_handler) task.
    ///     2. Remove the channels used to communicate with the [`Communicator's`](Communicator) [`handler`](Communicator::handler).
    pub async fn remove_handler(self: &Arc<Self>, handler_id: u64) -> Result<(), MCManageError> {
        if (self.communicator.lock().await).is_none() {
            error!(self.name, "The Communicator has not yet been set.");
            return Err(MCManageError::NotReady);
        }

        for (i, slot) in (*self.handler_send.lock().await).iter_mut().enumerate() {
            if i as u64 == handler_id {
                if let Some(slot) = self.handlers.lock().await.get_mut(handler_id as usize) {
                    if let Some(handler) = slot {
                        handler.abort();
                    }
                } else {
                    error!(self.name, "Found a channel but not a handler. This will be fixed.");
                }
                *slot = None;
                return Ok(());
            }
        }
        Err(MCManageError::NotFound)
    }

    /// This method continuously receives [`Messages`](Message) from one of the [`Communicator's`](Communicator) [`handlers`](Communicator::handler) and sends them to the
    /// specified [`handler`](Communicator::handler) if it can be found. In the event that the specified [`handler`](Communicator::handler) cannot be found, the
    /// [`Message`](Message) gets destroyed.
    async fn recv_handler(self: Arc<Self>, mut handler_recv: Receiver<Message>) -> Result<(), MCManageError> {
        loop {
            if let Some(message) = handler_recv.recv().await {
                let message_receiver =  message.receiver();
                if 0 == message.receiver() {
                    let communicator = self.communicator.lock().await.as_ref().expect("This handler would have never started if the communicator had not been set.").clone();
                    if let Ok(client_type) = communicator.get_client_type(message.sender()).await {
                        message.execute(&client_type, &communicator).await;
                    }
                } else {
                    for (i, slot) in (*self.handler_send.lock().await).iter_mut().enumerate() {
                        if message_receiver == i as u64 {
                            if let Some(handler) = slot.clone() {
                                if (handler.send(message).await).is_err() {
                                    error!(self.name, "A handler disconnected without removing itself from the InterCom. This will be fixed.");
                                    *slot = None
                                }
                            } else {
                                error!(self.name, "A handler tried to send a message to a handler which is not registered. The message will be destroyed.");
                            }
                            break;
                        }
                    }
                }
            } else {
                error!(self.name, "The assigned channel got closed before the handler got closed. This handler will now abort.");
                return Err(MCManageError::FatalError);
            }
        }
    }
    /// This method will continuously receive [`Messages`](Message) from the [`Communicator's`](Communicator)
    /// [`send_message`](common::communicator::CommunicatorTrait::send_message) method and send them to the specified handler. \
    /// \
    /// Note: There are two cases in which the message will be destroyed:
    ///     1. The message was directed at the main application.
    ///     2. The specified receiver was not yet registered.
    async fn main(self: Arc<Self>, mut bootup_result: Option<oneshot::Sender<()>>) -> Result<(), MCManageError> {
        self.send_start_result(&mut bootup_result).await;

        loop {
            if let Some(message) = self.receiver.lock().await.recv().await {
                if 0 == message.receiver() {
                    error!(self.name, "This application wanted to send a message to itself. This message will be destroyed.");
                    continue;
                }

                if let Some(slot) = self.handler_send.lock().await.get_mut(message.receiver() as usize) {
                    if let Some(handler) = slot.clone() {
                        if (handler.send(message).await).is_err() {
                            error!(self.name, "A handler disconnected without removing itself from the InterCom. This will be fixed.");
                            *slot = None
                        }
                        continue;
                    }
                }
                error!(self.name, "The Communicator tried to send a message to a handler which is not registered. The message will be destroyed.");
            } else {
                error!(self.name, "The Communicator disconnected! The InterCom will now shutdown too.");
                self.stop();
                return Err(MCManageError::FatalError);
            }
        }
    }
}