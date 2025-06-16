use actix::{Actor, Context, Handler, Message, Recipient};
use std::collections::HashMap;
use uuid::Uuid;

// ====== Mensagens ======

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct UserMessage {
    pub user_id: Uuid,
    pub message: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct ConnectUser {
    pub user_id: Uuid,
    pub addr: Recipient<WsMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DisconnectUser {
    pub user_id: Uuid,
}

// ====== WsServer Actor ======

pub struct WsServer {
    sessions: HashMap<Uuid, Recipient<WsMessage>>,
}

impl WsServer {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn send_message_to_user(&self, user_id: Uuid, message: String) {
        if let Some(recipient) = self.sessions.get(&user_id) {
            let _ = recipient.do_send(WsMessage(message.clone()));
        }
    }

    pub fn broadcast(&self, message: String) {
        for recipient in self.sessions.values() {
            let _ = recipient.do_send(WsMessage(message.clone()));
        }
    }
}

impl Actor for WsServer {
    type Context = Context<Self>;
}

// ====== Handlers ======

impl Handler<ConnectUser> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: ConnectUser, _ctx: &mut Self::Context) {
        println!("🟢 Usuário conectado ao WS: {:?}", msg.user_id);
        self.sessions.insert(msg.user_id, msg.addr);
    }
}

impl Handler<DisconnectUser> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: DisconnectUser, _ctx: &mut Self::Context) {
        println!("🔴 Usuário desconectado do WS: {:?}", msg.user_id);
        self.sessions.remove(&msg.user_id);
    }
}

impl Handler<UserMessage> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: UserMessage, _ctx: &mut Self::Context) {
        if let Some(recipient) = self.sessions.get(&msg.user_id) {
            let _ = recipient.do_send(WsMessage(msg.message.clone()));
        }
    }
}

impl Handler<BroadcastMessage> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _ctx: &mut Self::Context) {
        for recipient in self.sessions.values() {
            let _ = recipient.do_send(WsMessage(msg.0.clone()));
        }
    }
}

// use actix::{Actor, Context, Handler, Message, Recipient};
// use std::collections::HashMap;
// use uuid::Uuid;

// pub struct WsServer {
//     sessions: HashMap<Uuid, Recipient<WsMessage>>,
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct UserMessage {
//     pub user_id: Uuid,
//     pub message: String,
// }

// #[derive(Message)]
// #[rtype(result = "()")]
// pub struct BroadcastMessage(pub String);

// impl WsServer {
//     pub fn new() -> Self {
//         Self {
//             sessions: HashMap::new(),
//         }
//     }

//     pub fn send_message_to_user(&self, user_id: Uuid, message: String) {
//         if let Some(recipient) = self.sessions.get(&user_id) {
//             let _ = recipient.do_send(WsMessage(message.clone()));
//         }
//     }

//     pub fn broadcast(&self, message: String) {
//         for recipient in self.sessions.values() {
//             let _ = recipient.do_send(WsMessage(message.clone()));
//         }
//     }
// }

// impl Handler<UserMessage> for WsServer {
//     type Result = ();

//     fn handle(&mut self, msg: UserMessage, _ctx: &mut Self::Context) {
//         if let Some(recipient) = self.sessions.get(&msg.user_id) {
//             let _ = recipient.do_send(WsMessage(msg.message.clone()));
//         }
//     }
// }

// impl Handler<BroadcastMessage> for WsServer {
//     type Result = ();

//     fn handle(&mut self, msg: BroadcastMessage, _ctx: &mut Self::Context) {
//         for recipient in self.sessions.values() {
//             let _ = recipient.do_send(WsMessage(msg.0.clone()));
//         }
//     }
// }

// impl Actor for WsServer {
//     type Context = Context<Self>;
// }

// #[derive(Message, Clone)]
// #[rtype(result = "()")]
// #[allow(dead_code)]
// pub struct WsMessage(pub String);
