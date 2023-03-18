//! This module provides the implementation of the [`macro@ConcurrentClass`](super::ConcurrentClass) macro.


use proc_macro::{
    TokenStream,
    self
};
use quote::quote;
use syn::{
    DeriveInput,
    parse_macro_input
};


/// This function implements the [`macro@ConcurrentClass`](super::ConcurrentClass) macro.
pub fn concurrent_class(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: struct_name_ident,
        data: _,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let where_clause = &generics.where_clause;

    // implement the code generated for this enum
    quote! {
        impl #generics #struct_name_ident #generics #where_clause {
            /// This is the blocking implementation to restart a given struct. \
            /// For a non-blocking mode use the [`restart method`](Self::restart).
            pub async fn impl_restart(self: std::sync::Arc<Self>) -> Result<(), MCManageError> {
                self.check_allowed_restart().await?;
                
                let restart_time = std::time::Instant::now();

                info!(self.name, "Restarting...");


                // ### STOPPING ###
                loop {
                    match self.clone().impl_stop(true, true).await {
                        Ok(_) => {
                            break;
                        }
                        Err(erro) => {
                            match erro {
                                MCManageError::FatalError => {
                                    break;
                                }
                                _ => {
                                    tokio::time::sleep(*self.config.refresh_rate()).await;
                                }
                            }
                        }
                    }
                }
                self.reset().await;
                *self.status.lock().await = Status::Restarting;


                // ### STARTING ###

                // Try to start the class until it succeeds or the fail limit is reached
                let mut failcounter = 0;
                loop {
                    if let Err(erro) = self.clone().impl_start(true).await {
                        if failcounter == *self.config.max_tries() {
                            erro!(self.name, "The maximum number of start attempts has been reached. This struct will no longer attempt to start.");
                            self.reset().await;
                            return Err(MCManageError::FatalError);
                        } else {
                            *self.status.lock().await = Status::Restarting;
                            failcounter += 1;
                            erro!(self.name, "Encountered an error while starting. Error: {}", erro);
                            erro!(self.name, "This was attempt number {} out of {}", failcounter, self.config.max_tries());
                        }
                        tokio::time::sleep(*self.config.refresh_rate()).await;
                    } else {
                        break;
                    }
                }
                *self.status.lock().await = Status::Started;

                info!(self.name, "Restarted in {:.3} secs!", restart_time.elapsed().as_secs_f64());
                return Ok(());
            }
            /// Start a given struct without blocking the calling thread. \
            /// For a blocking mode use the [`impl_start method`](Self::impl_start).
            pub fn start(self: &std::sync::Arc<Self>) {
                tokio::spawn(self.clone().impl_start(false));
            }
            /// Stop a given struct without blocking the calling thread. \
            /// For a blocking mode use the [`impl_stop method`](Self::impl_stop).
            pub fn stop(self: &std::sync::Arc<Self>) {
                tokio::spawn(self.clone().impl_stop(false, true));
            }
            /// Restart a given struct without blocking the calling thread. \
            /// For a blocking mode use the [`impl_restart method`](Self::impl_restart).
            pub fn restart(self: &std::sync::Arc<Self>) {
                tokio::spawn(self.clone().impl_restart());
            }
            
            /// Wait for the started signal.
            async fn recv_start_result(self: &std::sync::Arc<Self>, bootup_result: tokio::sync::oneshot::Receiver<()>) -> Result<(), MCManageError> {
                if let Err(_) = bootup_result.await {
                    if let Status::Stopping = *self.status.lock().await {
                    } else {
                        erro!(self.name, "The main thread crashed. This struct could not be started.");
                        self.reset().await;
                        return Err(MCManageError::FatalError);
                    }
                }
                Ok(())
            }
            /// Send the started signal.
            async fn send_start_result(self: &std::sync::Arc<Self>, bootup_result: &mut Option<tokio::sync::oneshot::Sender<()>>) -> Result<(), MCManageError> {
                if let Err(_) = bootup_result.take().expect("The 'bootup_result' channel should only be taken once. Before taking it again this struct should be reset.").send(()) {            
                    erro!(self.name, "The thread starting the struct got stopped! This struct will now shut down.");
                    self.stop();
                    return Err(MCManageError::FatalError);
                }
                Ok(())
            }
            /// Check if the [`impl_start`](Self::impl_start) method is allowed to be executed. \
            /// This function will also set the status of the given class to the right value.
            /// 
            /// # Returns
            /// 
            /// | Return                                | Description                                               |
            /// |---------------------------------------|-----------------------------------------------------------|
            /// | `Ok(())`                              | The method can be executed immediately.                   |
            /// | [`MCManageError::AlreadyExecuted`]    | The method has already been executed.                     |
            /// | [`MCManageError::CurrentlyExecuting`] | The method is currently being executed by another thread. |
            /// | [`MCManageError::NotReady`]           | The method can not be used.                               |
            async fn check_allowed_start(self: &std::sync::Arc<Self>, restart: bool) -> Result<(), MCManageError> {
                let mut status = *self.status.lock().await;
                match status {
                    Status::Started => return Err(MCManageError::AlreadyExecuted),
                    Status::Starting => return Err(MCManageError::CurrentlyExecuting),
                    Status::Stopped => {
                        status = Status::Starting;
                        return Ok(())
                    },
                    Status::Stopping => return Err(MCManageError::NotReady),
                    Status::Restarting => {
                        if !restart {
                            return Err(MCManageError::CurrentlyExecuting)
                        } else {
                            return Ok(())
                        }
                    }
                }
            }
            /// Check if the [`impl_stop`](Self::impl_stop) method is allowed to be executed. \
            /// This function will also set the status of the given class to the right value. \
            /// If the `forced` parameter got set to true this function will wait until the class has either started or stopped.
            /// 
            /// # Returns
            /// 
            /// | Return                                | Description                                               |
            /// |---------------------------------------|-----------------------------------------------------------|
            /// | `Ok(())`                              | The method can be executed immediately.                   |
            /// | [`MCManageError::AlreadyExecuted`]    | The method has already been executed.                     |
            /// | [`MCManageError::CurrentlyExecuting`] | The method is currently being executed by another thread. |
            /// | [`MCManageError::NotReady`]           | The method can not be used.                               |
            async fn check_allowed_stop(self: &std::sync::Arc<Self>, restart: bool, forced: bool) -> Result<(), MCManageError> {
                if forced && !restart {
                    // wait till the class has started
                    loop {
                        if let Status::Started = *self.status.lock().await {
                            break;
                        }
                        tokio::time::sleep(*self.config.refresh_rate()).await;
                    }
                }
                
                let mut status = *self.status.lock().await;
                match status {
                    Status::Started => {
                        status = Status::Stopping;
                        return Ok(())
                    }
                    Status::Starting => return Err(MCManageError::NotReady),
                    Status::Stopped => return Err(MCManageError::AlreadyExecuted),
                    Status::Stopping => return Err(MCManageError::CurrentlyExecuting),
                    Status::Restarting => {
                        if !restart {
                            return Err(MCManageError::NotReady)
                        } else {
                            return Ok(())
                        }
                    }
                }
            }
            /// Check if the [`impl_restart`](Self::impl_restart) method is allowed to be executed. \
            /// This function will also set the status of the given class to the right value.
            /// 
            /// # Returns
            /// 
            /// | Return                                | Description                                                               |
            /// |---------------------------------------|---------------------------------------------------------------------------|
            /// | `Ok(())`                              | The method can be executed immediately.                                   |
            /// | [`MCManageError::NotStarted`]         | The method can not be executed since the given struct is not yet started. |
            /// | [`MCManageError::CurrentlyExecuting`] | The method is currently being executed by another thread.                 |
            /// | [`MCManageError::NotReady`]           | The method can not be used.                                               |
            async fn check_allowed_restart(self: &std::sync::Arc<Self>) -> Result<(), MCManageError> {
                let mut status = *self.status.lock().await;
                match status {
                    Status::Started => {
                        status = Status::Restarting;
                        return Ok(())
                    }
                    Status::Starting => return Err(MCManageError::NotReady),
                    Status::Stopped => return Err(MCManageError::NotStarted),
                    Status::Stopping => return Err(MCManageError::NotStarted),
                    Status::Restarting => return Err(MCManageError::CurrentlyExecuting),
                }
            }
            /// Start the [`main thread`](Self::main) of this struct.
            async fn start_main_thread(self: &std::sync::Arc<Self>) -> Result<tokio::sync::oneshot::Receiver::<()>, MCManageError> {
                let (tx, rx) = tokio::sync::oneshot::channel();
                let mut main_thread = self.main_thread.lock().await;
                if let None = *main_thread {
                    *main_thread = Some(tokio::task::spawn(self.clone().main(Some(tx))));
                    Ok(rx)
                } else {
                    erro!(self.name, "There was already a main thread assigned. This struct will be reset.");
                    self.reset().await;
                    Err(MCManageError::FatalError)
                }
            }
            /// Stop the [`main thread`](Self::main) of this struct.
            async fn stop_main_thread(self: &std::sync::Arc<Self>) -> Result<(), MCManageError> {
                if let Some(thread) = self.main_thread.lock().await.take() {
                    thread.abort();
                    Ok(())
                } else {
                    erro!(self.name, "Failed to find the main thread. This struct will be reset.");
                    self.reset().await;
                    Err(MCManageError::NotFound)
                }
            }
        }
    }
    .into()
}