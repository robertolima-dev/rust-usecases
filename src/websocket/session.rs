use actix::{Actor, ActorContext, StreamHandler};
use actix_web_actors::ws;
use uuid::Uuid;

pub struct WsSession {
    pub user_id: Uuid,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("🟢 Mensagem recebida de {}: {}", self.user_id, text);
                ctx.text(format!("Você disse: {}", text));
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(reason)) => {
                println!("🔴 Conexão encerrada de {}", self.user_id);
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
