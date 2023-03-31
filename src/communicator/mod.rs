//! This module provides the [`Communicator`] of the MCManage application. \
//! It accepts new connections from the MCManage network and manages the sending and receiving of [`messages`](common::communicator::message::Message).


use std::{
    net::{
        IpAddr,
        Ipv4Addr,
        SocketAddr
    },
    sync::Arc,
    time::{
        Instant,
        Duration
    },
};

use async_trait::async_trait;
use proc_macros::ConcurrentClass;
use tokio::{
    io::{
        AsyncWriteExt,
        self
    },
    net::{
        tcp::{
            ReadHalf,
            WriteHalf
        },
        TcpStream,
        TcpListener
    },
    spawn,
    time::sleep,
    try_join
};
use tokio::sync::{
    Mutex,
    mpsc::{
        channel,
        Sender,
        Receiver
    },
    oneshot
};
use common::{
    communicator::{
        client_type::ClientType,
        message::{
            command::{
                Command,
                get_type::GetTypeArgs,
                set_id::SetIdArgs
            },
            Message,
            message_type::MessageType
        },
        CommunicatorTrait
    },
    config::Config,
    error,
    info,
    mcmanage_error::MCManageError,
    status::Status,
    types::ThreadJoinHandle
};

use self::{
    reserved_connections::ReservedConnections,
    intercom::InterCom
};


mod tests;
mod reserved_connections;
mod intercom;


// TODO send encrypted messages

/// This struct manages the communication between this application and other ones connected to it via a socket connection. In this case, there are two kinds of connected
/// clients: the [`Worker`](ClientType::Worker) and the [`User`](ClientType::User). For every new client, a new [`handler`](Self::handler)
/// gets started, which is responsible for sending [`Messages`](common::communicator::message::Message) stored inside the internal buffer and executing the ones received from
/// the client.
#[derive(ConcurrentClass)]
pub struct Communicator {
    /// This struct's name
    name: String,
    /// The applications [`Config`]
    config: Arc<Mutex<Config>>,
    /// The main thread of this struct
    main_thread: Arc<Mutex<Option<ThreadJoinHandle>>>,
    /// The [`Status`] of this struct
    status: Mutex<Status>,

    /// This holds the [`InterCom`]
    intercom: Arc<InterCom>,
    /// This is the sending channel for sending messages to the [`InterCom`]
    messages_send: Sender<Message>,
    /// A list of all users
    users: Mutex<Vec<Option<(u64, ClientType)>>>,
    /// This holds a struct, which is in itself a list of all workers
    workers: ReservedConnections
}
#[async_trait]
impl CommunicatorTrait for Communicator {
    async fn send_message(self: &Arc<Self>, message: Message) {
        self.messages_send.send(message).await.expect("The receiver channel should be available since it is held by the same struct as the sender channel.")
    }
}
impl Communicator {
    /// Create a new [`Communicator`] instance.
    pub async fn new(config: &Arc<Mutex<Config>>) -> Arc<Self> {
        let (messages_send, messages_recv) = channel(Config::buffsize(config).await);
        let communicator = Arc::new(Self {
            name: "Communicator".to_string(),
            config: config.clone(),
            main_thread:Arc::new(None.into()),
            status: Mutex::new(Status::Stopped),

            intercom: InterCom::new(config, messages_recv).await,
            messages_send,
            users: vec![].into(),
            workers: ReservedConnections::new()
        });
        communicator.intercom.set_communicator(&communicator).await;

        communicator
    }

    /// This is the blocking implementation to start a given struct. \
    /// For a non-blocking mode use the [`start method`](Self::start). \
    /// \
    /// The `restart` parameter will be used by the [`restart method`](Self::impl_restart) to deactivate all non-fatal error messages of this method and to enable
    /// this method to be executed during a restart.
    pub async fn impl_start(self: Arc<Self>, restart: bool) -> Result<(), MCManageError> {
        self.check_allowed_start(restart).await?;

        if !restart { info!(self.name, "Starting..."); }
        let start_time = Instant::now();

        if !restart {
            self.intercom.clone().impl_start(restart).await?;
        }

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

        if restart {
            self.intercom.clone().impl_restart().await?;
        } else {
            self.intercom.clone().impl_stop(restart, forced).await?;
        }

        self.stop_main_thread().await;
        *self.users.lock().await = vec![];
        *self.status.lock().await = Status::Stopped;

        if !restart { info!(self.name, "Stopped in {:.3} secs!", stop_time.elapsed().as_secs_f64()); }
        Ok(())
    }
    /// Reset a given struct to its starting values.
    async fn reset(self: &Arc<Self>) {
        if let Some(thread) = self.main_thread.lock().await.take() {thread.abort();}
        *self.status.lock().await = Status::Stopped;
        *self.users.lock().await = vec![];
    }
    /// This represents the main loop of a given struct.
    async fn main(self: Arc<Self>, mut bootup_result: Option<oneshot::Sender<()>>) -> Result<(), MCManageError> {
        let mut handlers = vec![];

        let mut tries = 0;
        loop {
            tries += 1;

            match TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), Config::communicator_port(&self.config).await)).await {
                Ok(tcplistener) => {
                    self.send_start_result(&mut bootup_result).await;

                    // the main loop of the tcplistener
                    loop {
                        match tcplistener.accept().await {
                            Ok(client) => {
                                handlers.push(spawn(self.clone().handler(client.0, client.1)));
                            }
                            Err(erro) => {
                                error!(self.name, "Found an error while accepting a new client. This connection will be closed. Error: {erro}");
                                // It is now the clients responsibility to retry the connection
                            }
                        }
                    }
                }
                Err(erro) => {
                    let max_tries = Config::max_tries(&self.config).await;
                    if tries == max_tries {
                        drop(bootup_result.take().expect("The 'bootup_result' channel only be should taken once. Before taking it again the Communicator should be reset."));

                        error!(self.name, "The maximum number of tries has been reached. A reset will be performed.");
                        self.reset().await;
                        return Err(MCManageError::FatalError);
                    }
                    else {
                        error!(self.name, "Received an error when trying to bind the socket server. Error: {erro}");
                        error!(self.name, "This was try {tries} of of {}. 3 seconds till the next one.", max_tries);
                        sleep(Duration::new(3, 0)).await;
                    }
                }
            }
        }
    }
    /// Get the [`ClientType`] of a given client.
    async fn get_client_type(self: &Arc<Self>, client_id: u64) -> Result<ClientType, MCManageError> {
        if client_id as usize > self.users.lock().await.len() {
            return Err(MCManageError::NotFound);
        }

        for client in (*self.users.lock().await).iter().flatten() {
            if client_id == client.0 {
                return Ok(client.1.clone());
            }
        }
        Err(MCManageError::NotFound)
    }
    /// This method does two things.
    ///     1. calls the [`get_client_type`](Communicator::get_client_type) method to get the client's type.
    ///     2. gets an id for the client
    /// 
    /// If the client acts invalidly, the connection will be closed and the [`handler`](Communicator::handler) will abort.
    async fn register_client(self: &Arc<Self>, client: &mut TcpStream, client_addr: &SocketAddr) -> Result<(u64, ClientType), MCManageError> {
        let message: Vec<u8> = Message::new(Command::GetType(GetTypeArgs{client_type:None}), MessageType::Request, 0, 0).try_into()?;
        client.write_all(message.as_slice()).await?;

        let message;
        let mut buffer: Vec<u8> = vec![0; Config::buffsize(&self.config).await];
        loop {
            client.readable().await?;
            match client.try_read(&mut buffer) {
                Ok(0) => {
                    info!(self.name, "{client_addr} disconnected.");
                    return Err(MCManageError::CriticalError);
                }
                Ok(_) => {
                    message = Message::try_from(buffer.to_vec())?;
                    break;
                }
                Err(ref erro) if erro.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(erro) => {
                    error!(self.name, "Encountered an error while receiving a message from {client_addr}. This connection will be closed. Error: {erro}");
                    return Err(MCManageError::CriticalError)
                }
            }
        }

        if let Command::GetType(args) = message.command() {
            if let Some(ClientType::Worker) = args.client_type {
                // return id here
                todo!("Validate and register the worker.") // TODO Validate and register the worker.
            } else if let Some(ClientType::User) = args.client_type {
                let mut id_set = false;
                let mut id = 1 + self.workers.count() as u64;
                let mut clients = self.users.lock().await;
                for slot in &mut *clients {
                    if slot.is_none() {
                        *slot = Some((id, ClientType::User));
                        id_set = true;
                        break;
                    }
                    id+=1;
                }
                if !id_set {
                    clients.push(Some((id, ClientType::User)));
                }

                let message: Vec<u8> = Message::new(Command::SetId(SetIdArgs{id}), MessageType::Request, 0, 0).try_into()?;
                client.write_all(message.as_slice()).await?;

                return Ok((id, ClientType::User));
            }
        }

        error!(self.name, "The client did not react as expected. This connection will be closed.");
        Err(MCManageError::InvalidClient)
    }
    /// This method represents a handler, and it starts two asynchronous tasks that both run in parallel to handle different tasks of a handler:
    ///     1. The first method [`handle_client_send`](Communicator::handle_client_send) sends messages to the client.
    ///     2. The second method [`handle_client_recv`](Communicator::handle_client_recv) executes messages received by the client.
    async fn handler(self: Arc<Self>, mut client: TcpStream, client_addr: SocketAddr) -> Result<(), MCManageError> {
        info!(self.name, "A new client has connected using the IP address `{}`.", client_addr);

        let (client_id, client_type) = self.register_client(&mut client, &client_addr).await?;
        let (intercom_send, intercom_recv) = self.intercom.add_handler(client_id).await?;

        let (client_read, mut client_write) = client.split();

        let handle_result = try_join!(
            self.clone().handle_client_send(&mut client_write, intercom_recv),
            self.clone().handle_client_recv(&client_read, intercom_send, client_addr, client_type)
        );
        client_write.shutdown().await?;

        if (self.intercom.remove_handler(client_id).await).is_err() {
            // The NotReady error will not be returned since the add_handler method would have send it
            // There is no need to handle the NotFound error since this is the result this method should achieve
        }
        for user in &mut *self.users.lock().await {
            if let Some((id, _)) = *user {
                if client_id == id {
                    *user = None;
                    break;
                }
            }
        }

        if let Err(erro) = handle_result {
            if let MCManageError::CriticalError = erro {
                return Err(erro);
            } else if let MCManageError::ClientError = erro {
            } else {
                error!(self.name, "Encountered an error while communicating with {client_addr}. This connection will be closed. Error: {erro}");
            }
        } else {
            info!(self.name, "{client_addr} disconnected.");
        }
        Ok(())
    }
    /// This method is responsible for sending messages to the client.
    async fn handle_client_send<'a>(self: Arc<Self>, client: &mut WriteHalf<'a>, mut intercom_recv: Receiver<Message>) -> Result<(), MCManageError> {
        loop {
            if let Some(message) = intercom_recv.recv().await {
                let message: Vec<u8> = message.try_into()?;
                client.write_all(message.as_slice()).await?;
            } else { // Covers only TryRecvError::Disconnect
                panic!("The sender channel should be available since it is held by the same struct as the receiver channel.");
            }
        }
    }
    /// This method is responsible executing messages received by the client.
    async fn handle_client_recv<'a>(self: Arc<Self>, client: &ReadHalf<'a>, intercom_send: Sender<Message>, client_addr: SocketAddr, client_type: ClientType) -> Result<(), MCManageError> {
        loop {
            let mut buffer: Vec<u8> = vec![0; Config::buffsize(&self.config).await];
            loop {
                client.readable().await?;
                match client.try_read(&mut buffer) {
                    Ok(0) => {
                        info!(self.name, "{client_addr} disconnected.");
                        return Err(MCManageError::ClientError);
                    }
                    Ok(_) => {
                        let message = Message::try_from(buffer.to_vec())?;
                        // 0 is always the MCManage main application
                        if 0 == message.receiver() {
                            message.execute(&client_type, &self).await;
                        } else if (intercom_send.send(message).await).is_err() {
                            error!(self.name, "The InterCom disconnected. The Communicator will restart.");
                            self.restart();
                            return Err(MCManageError::CriticalError);
                        }
                    }
                    Err(ref erro) if erro.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(erro) => {
                        error!(self.name, "Encountered an error while receiving a message from {client_addr}. This connection will be closed. Error: {erro}");
                        return Err(MCManageError::ClientError)
                    }
                }
            }
        }
    }
}