use actix::{Actor, Context, Handler, Message, Recipient};
use std::collections::HashMap;
use uuid::Uuid;

pub struct WsServer {
    sessions: HashMap<Uuid, Recipient<WsMessage>>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct UserMessage {
    pub user_id: Uuid,
    pub message: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub String);

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

impl Actor for WsServer {
    type Context = Context<Self>;
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
#[allow(dead_code)]
pub struct WsMessage(pub String);
