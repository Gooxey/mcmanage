#![cfg(test)]


use common::{
    communicator::message::{
        command::{
            Command,
            set_id::SetIdArgs
        },
        message_type::MessageType
    }
};

use crate::test_functions::*;

use super::*;


async fn new_intercom() -> (Arc<InterCom>, Sender<Message>) {
    setup_logger();
    let config = Config::new().await;
    let (send_message, rx) = channel(Config::buffsize(&config).await);
    let intercom = InterCom::new(&config, rx).await;

    intercom.set_communicator(&Communicator::new(&config).await).await;

    (intercom, send_message)
}


#[tokio::test]
async fn add_single_handler() {
    let (intercom, _) = new_intercom().await;

    let (_, _) = intercom.add_handler(3).await.unwrap();

    let handlers = intercom.handler_send.lock().await;
    assert_eq!(4, handlers.len(), "handlers should contain 4 items");
    if let Some(_) = handlers[0] {
        assert!(false, "the first item should be None");
    }
    if let Some(_) = handlers[1] {
        assert!(false, "the second item should be None");
    }
    if let Some(_) = handlers[2] {
        assert!(false, "the third item should be None");
    };
    if let None = handlers[3] {
        assert!(false, "the fourth item should be Some");
    }
    cleanup();
}
#[tokio::test]
async fn add_multiple_handlers() {
    let (intercom, _) = new_intercom().await;

    let (_, _) = intercom.add_handler(3).await.unwrap();
    let (_, _) = intercom.add_handler(5).await.unwrap();

    let handlers = intercom.handler_send.lock().await;
    assert_eq!(6, handlers.len(), "handlers should contain 6 items");
    if let Some(_) = handlers[0] {
        assert!(false, "the first item should be None");
    }
    if let Some(_) = handlers[1] {
        assert!(false, "the second item should be None");
    }
    if let Some(_) = handlers[2] {
        assert!(false, "the third item should be None");
    };
    if let None = handlers[3] {
        assert!(false, "the fourth item should be Some");
    }
    if let Some(_) = handlers[4] {
        assert!(false, "the fifth item should be None");
    };
    if let None = handlers[5] {
        assert!(false, "the sixth item should be Some");
    };
    cleanup();
}
#[tokio::test]
async fn add_multiple_handlers_reversed() {
    let (intercom, _) = new_intercom().await;

    let (_, _) = intercom.add_handler(5).await.unwrap();
    let (_, _) = intercom.add_handler(3).await.unwrap();

    let handlers = intercom.handler_send.lock().await;
    assert_eq!(6, handlers.len(), "handlers should contain 6 items");
    if let Some(_) = handlers[0] {
        assert!(false, "the first item should be None");
    }
    if let Some(_) = handlers[1] {
        assert!(false, "the second item should be None");
    }
    if let Some(_) = handlers[2] {
        assert!(false, "the third item should be None");
    };
    if let None = handlers[3] {
        assert!(false, "the fourth item should be Some");
    }
    if let Some(_) = handlers[4] {
        assert!(false, "the fifth item should be None");
    };
    if let None = handlers[5] {
        assert!(false, "the sixth item should be Some");
    };
    cleanup();
}

#[tokio::test]
async fn remove_single_handler() {
    let (intercom, _) = new_intercom().await;

    let (_, _) = intercom.add_handler(3).await.unwrap();
    intercom.remove_handler(3).await.unwrap();

    let handlers = intercom.handler_send.lock().await;
    assert_eq!(4, handlers.len(), "handlers should contain 4 items");
    for handler in &*handlers {
        if let Some(_) = handler {
            assert!(false, "No handler should be registered.")
        }
    }
    cleanup();
}
#[tokio::test]
async fn remove_multiple_handlers() {
    let (intercom, _) = new_intercom().await;

    let (_, _) = intercom.add_handler(3).await.unwrap();
    let (_, _) = intercom.add_handler(5).await.unwrap();
    intercom.remove_handler(3).await.unwrap();
    intercom.remove_handler(5).await.unwrap();

    let handlers = intercom.handler_send.lock().await;
    assert_eq!(6, handlers.len(), "handlers should contain 6 items");
    for handler in &*handlers {
        if let Some(_) = handler {
            assert!(false, "No handler should be registered.")
        }
    }
    cleanup();
}
#[tokio::test]
async fn remove_multiple_handlers_reversed() {
    let (intercom, _) = new_intercom().await;

    let (_, _) = intercom.add_handler(5).await.unwrap();
    let (_, _) = intercom.add_handler(3).await.unwrap();
    intercom.remove_handler(5).await.unwrap();
    intercom.remove_handler(3).await.unwrap();

    let handlers = intercom.handler_send.lock().await;
    assert_eq!(6, handlers.len(), "handlers should contain 6 items");
    for handler in &*handlers {
        if let Some(_) = handler {
            assert!(false, "No handler should be registered.")
        }
    }
    cleanup();
}

#[tokio::test]
async fn add_remove_add_handler() {
    let (intercom, _) = new_intercom().await;

    let (_, _) = intercom.add_handler(3).await.unwrap();
    intercom.remove_handler(3).await.unwrap();
    let (_, _) = intercom.add_handler(3).await.unwrap();

    let handlers = intercom.handler_send.lock().await;
    assert_eq!(4, handlers.len(), "handlers should contain 4 items");
    if let Some(_) = handlers[0] {
        assert!(false, "the first item should be None");
    }
    if let Some(_) = handlers[1] {
        assert!(false, "the second item should be None");
    }
    if let Some(_) = handlers[2] {
        assert!(false, "the third item should be None");
    };
    if let None = handlers[3] {
        assert!(false, "the fourth item should be Some");
    }
    cleanup();
}

#[tokio::test]
async fn communicator_to_handler() {
    let (intercom, send_message) = new_intercom().await;
    intercom.clone().impl_start(false).await.unwrap();
    let (_handler_send, mut handler_recv) = intercom.add_handler(3).await.unwrap();

    let message = Message::new(Command::SetId(SetIdArgs{id:0}), MessageType::Request, 3, 0); // The command here does not matter
    send_message.send(message.clone()).await.unwrap();
    assert_eq!(message, handler_recv.recv().await.unwrap());

    intercom.impl_stop(false, true).await.unwrap();
    cleanup();
}
#[tokio::test]
async fn handler_to_handler() {
    let (intercom, _send_message) = new_intercom().await;
    intercom.clone().impl_start(false).await.unwrap();
    let (handler_send, _handler_recv) = intercom.add_handler(3).await.unwrap();
    let (_handler2_send, mut handler2_recv) = intercom.add_handler(5).await.unwrap();

    let message = Message::new(Command::SetId(SetIdArgs{id:0}), MessageType::Request, 5, 0); // The command here does not matter
    handler_send.send(message.clone()).await.unwrap();
    assert_eq!(message, handler2_recv.recv().await.unwrap());


    intercom.impl_stop(false, true).await.unwrap();
    cleanup();
}