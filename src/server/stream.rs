use crate::services::realtime::realtime;
use actix::{Actor, StreamHandler};
use actix_web::{Error, HttpRequest, HttpResponse, get, web};
use actix_web_actors::ws;
use ws::Message;

const CHUNK: usize = 1 << 16;

#[get("/realtime/ws")]
pub async fn realtime_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(RealtimeWs, &req, stream)
}

struct RealtimeWs;

impl Actor for RealtimeWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<Message, ws::ProtocolError>> for RealtimeWs {
    fn handle(&mut self, msg: Result<Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Text(txt)) => process_realtime_json(txt.as_bytes(), ctx),
            Ok(Message::Binary(bin)) => process_realtime_json(&bin, ctx),
            Ok(Message::Ping(bytes)) => ctx.pong(&bytes),
            Ok(Message::Close(_)) => ctx.close(None),
            _ => {}
        }
    }
}

fn process_realtime_json(json_bytes: &[u8], ctx: &mut ws::WebsocketContext<RealtimeWs>) {
    let game_data = match serde_json::from_slice(json_bytes) {
        Ok(v) => v,
        Err(e) => {
            ctx.text(e.to_string());
            return;
        }
    };

    let data = match realtime(&game_data) {
        Ok(v) => v,
        Err(e) => {
            ctx.text(e.as_str());
            return;
        }
    };

    let bin = match bincode::encode_to_vec(data, bincode::config::standard()) {
        Ok(v) => v,
        Err(e) => {
            ctx.text(e.to_string());
            return;
        }
    };

    let total = bin.len() as u64;
    ctx.binary(total.to_le_bytes().to_vec());

    for chunk in bin.chunks(CHUNK) {
        ctx.binary(chunk.to_vec());
    }
}
