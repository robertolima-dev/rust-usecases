use crate::websocket::server::{ConnectUser, DisconnectUser, WsMessage, WsServer};
use actix::ActorContext;
use actix::{Actor, AsyncContext, Handler, StreamHandler};
use actix_web_actors::ws;
use uuid::Uuid;
pub struct WsSession {
    pub user_id: Uuid,
    pub ws_server: actix::Addr<WsServer>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("✅ WS conectado: {:?}", self.user_id);

        // Registra o usuário no server
        self.ws_server.do_send(ConnectUser {
            user_id: self.user_id,
            addr: ctx.address().recipient(),
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        println!("🛑 WS desconectando: {:?}", self.user_id);

        // Remove o usuário do server
        self.ws_server.do_send(DisconnectUser {
            user_id: self.user_id,
        });

        actix::Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Text(text)) => {
                println!("📩 Mensagem recebida do usuário {}: {}", self.user_id, text);
                ctx.text(format!("Echo: {}", text)); // Pode customizar
            }
            Ok(ws::Message::Close(reason)) => {
                println!("🚪 WS fechando: {:?}", reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

// Permite ao server enviar mensagens para o usuário
impl Handler<WsMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

// use actix::{Actor, ActorContext, StreamHandler};
// use actix_web_actors::ws;
// use uuid::Uuid;

// pub struct WsSession {
//     pub user_id: Uuid,
// }

// impl Actor for WsSession {
//     type Context = ws::WebsocketContext<Self>;
// }

// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(ws::Message::Text(text)) => {
//                 println!("🟢 Mensagem recebida de {}: {}", self.user_id, text);
//                 ctx.text(format!("Você disse: {}", text));
//             }
//             Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
//             Ok(ws::Message::Close(reason)) => {
//                 println!("🔴 Conexão encerrada de {}", self.user_id);
//                 ctx.close(reason);
//                 ctx.stop();
//             }
//             _ => (),
//         }
//     }
// }
