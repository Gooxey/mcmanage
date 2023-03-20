#![cfg(test)]


use std::io;

use common::communicator::message::{command::{Command}, message_type::MessageType};
use tokio::join;

use super::*;


async fn recv_message(client: &mut TcpStream) -> Message {
    let mut buffer: Vec<u8> = vec![0; *Config::new().lock().await.buffsize()];
    loop {
        client.readable().await.unwrap();
        match client.try_read(&mut buffer) {
            Ok(0) => {
                assert!(false, "The Communicator disconnected.");
            }
            Ok(_) => {
                return Message::try_from(buffer.to_vec()).unwrap()
            }
            Err(ref erro) if erro.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(erro) => {
                assert!(false, "Encountered an error while receiving a message from the Communicator. Error: {erro}");
            }
        }
    }
}
async fn register_client(client: &mut TcpStream) -> u64 {
    let message = recv_message(client).await;
    assert_eq!(Command::GetType(GetTypeArgs { client_type: None }), *message.command());
    assert_eq!(MessageType::Request, *message.message_type());


    let message: Vec<u8> = Message::new(Command::GetType(GetTypeArgs{client_type:Some(ClientType::User)}), MessageType::Response, 0, 0).try_into().unwrap();
    client.write_all(message.as_slice()).await.unwrap();

    let message = recv_message(client).await;
    assert_eq!(MessageType::Request, *message.message_type());
    if let Command::SetId(args) = message.command() {
        args.id
    } else {
        assert!(false, "Expected to receive an id. Received: {:?}", message);
        0 // this can be any value because the assert above will panic
    }
}

#[tokio::test]
async fn start() {
    let communicator = Communicator::new().await;
    communicator.clone().impl_start(false).await.unwrap();

    assert_eq!(Status::Started, *communicator.status.lock().await);
    if let None = *communicator.main_thread.lock().await  {
        assert!(false, "Expected `main_thread` field to not be Some.");
    }

    communicator.impl_stop(false, true).await.unwrap();
}
#[tokio::test]
async fn stop() {
    let communicator = Communicator::new().await;
    communicator.clone().impl_start(false).await.unwrap();
    communicator.clone().impl_stop(false, true).await.unwrap();

    assert_eq!(Status::Stopped, *communicator.status.lock().await);
    if let Some(_) = *communicator.main_thread.lock().await  {
        assert!(false, "Expected `main_thread` field to not be None.");
    };
}
#[tokio::test]
async fn restart() {
    let communicator = Communicator::new().await;
    communicator.clone().impl_start(false).await.unwrap();
    communicator.clone().impl_restart().await.unwrap();

    assert_eq!(Status::Started, *communicator.status.lock().await);
    if let None = *communicator.main_thread.lock().await  {
        assert!(false, "Expected `main_thread` field to not be Some.");
    }

    communicator.impl_stop(false, true).await.unwrap();
}
#[tokio::test]
async fn handle_single_user() {
    let communicator = Communicator::new().await;
    communicator.clone().impl_start(false).await.unwrap();

    let mut client = TcpStream::connect(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), *Config::new().lock().await.communicator_port())).await.unwrap();

    let id = register_client(&mut client).await;
    assert_eq!(1, id, "Expected to receive the id 1 since this is the only client. There should also not be a single worker registered.");

    let message = Message::new(Command::SetId(SetIdArgs{id:0}), MessageType::Request, id, 1);
    communicator.send_message(message.clone()).await;

    assert_eq!(message, recv_message(&mut client).await, "The received message did not equal the one sent.");

    communicator.impl_stop(false, true).await.unwrap();
}

async fn client(id: &mut u64, communicator: Arc<Communicator>) {
    let mut client = TcpStream::connect(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), *Config::new().lock().await.communicator_port())).await.unwrap();

    *id = register_client(&mut client).await;
    if *id == 0 {
        assert!(false, "The id should not be 0 because that is the id of the main application.")
    }
    if *id > 2 {
        assert!(false, "The id should either be 1 or 2 because there are only two clients. Instead this id was received: {id}")
    }

    let message = Message::new(Command::SetId(SetIdArgs{id:0}), MessageType::Request, *id, 0);
    communicator.send_message(message.clone()).await;

    assert_eq!(message, recv_message(&mut client).await, "The received message did not equal the one sent.");
}
#[tokio::test]
async fn handle_multiple_users() {
    let communicator = Communicator::new().await;
    communicator.clone().impl_start(false).await.unwrap();

    let mut user_1 = 0;
    let mut user_2 = 0;

    join!(
        client(&mut user_1, communicator.clone()),
        client(&mut user_2, communicator.clone())
    );

    if user_1 == user_2 {
        assert!(false, "Expected both users to have different ids.")
    }

    communicator.impl_stop(false, true).await.unwrap();
}