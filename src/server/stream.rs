use crate::services::realtime::realtime;
use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, get, web};
use actix_web_actors::ws;
use std::io::{self, Write};
use std::mem;
use std::ptr;

const CHUNK: usize = 64 * 1024;

struct RealtimeWs;

#[get("/realtime/ws")]
pub async fn realtime_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(RealtimeWs, &req, stream)
}

impl Actor for RealtimeWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for RealtimeWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(txt)) => process_realtime_json(txt.as_bytes(), ctx),
            Ok(ws::Message::Binary(bin)) => process_realtime_json(&bin, ctx),
            Ok(ws::Message::Ping(bytes)) => ctx.pong(&bytes),
            Ok(ws::Message::Close(_)) => ctx.close(None),
            _ => {}
        }
    }
}

struct WsExactChunkWriter<'a> {
    ctx: &'a mut ws::WebsocketContext<RealtimeWs>,
    buf: Vec<u8>,
    remaining_total: usize,
    written_total: usize,
}

impl<'a> WsExactChunkWriter<'a> {
    #[inline]
    fn new(ctx: &'a mut ws::WebsocketContext<RealtimeWs>, total: usize) -> Self {
        let first = total.min(CHUNK);
        Self {
            ctx,
            buf: Vec::with_capacity(first),
            remaining_total: total,
            written_total: 0,
        }
    }

    #[inline]
    fn flush_buf(&mut self) {
        if self.buf.is_empty() {
            return;
        }
        // move sem copiar
        let out = mem::replace(&mut self.buf, Vec::new());
        self.ctx.binary(out);
    }

    #[inline]
    fn start_next_frame(&mut self) {
        if self.remaining_total == 0 {
            return;
        }
        let next = self.remaining_total.min(CHUNK);
        self.buf = Vec::with_capacity(next);
    }
}

impl<'a> Write for WsExactChunkWriter<'a> {
    #[inline]
    fn write(&mut self, mut src: &[u8]) -> io::Result<usize> {
        let mut total = 0;

        while !src.is_empty() {
            if self.buf.capacity() == 0 {
                self.start_next_frame();
            }

            let space = self.buf.capacity() - self.buf.len();
            if space == 0 {
                self.flush_buf();
                self.start_next_frame();
                continue;
            }

            let remain = self.remaining_total - self.written_total;
            if remain == 0 {
                break;
            }

            let take = space.min(src.len()).min(remain);
            unsafe {
                let dst_ptr = self.buf.as_mut_ptr().add(self.buf.len());
                ptr::copy_nonoverlapping(src.as_ptr(), dst_ptr, take);
                self.buf.set_len(self.buf.len() + take);
            }

            self.written_total += take;
            total += take;
            src = &src[take..];

            if self.buf.len() == self.buf.capacity() {
                self.flush_buf();
            }
        }

        Ok(total)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.flush_buf();
        Ok(())
    }
}

#[inline]
fn bin_err(ctx: &mut ws::WebsocketContext<RealtimeWs>, s: impl ToString) {
    ctx.binary(s.to_string().into_bytes());
}

fn process_realtime_json(json_bytes: &[u8], ctx: &mut ws::WebsocketContext<RealtimeWs>) {
    println!("Received json len: {}", json_bytes.len());

    let game_data = match serde_json::from_slice(json_bytes) {
        Ok(v) => v,
        Err(e) => {
            bin_err(ctx, e);
            return;
        }
    };

    println!("got game_data");

    let data = match realtime(&game_data) {
        Ok(v) => v,
        Err(e) => {
            bin_err(ctx, e.as_str());
            return;
        }
    };

    let sizeof_data = data.bincode_size();

    println!("bincode size: {}", sizeof_data);

    let void_data: &[u8] = &[0u8, 0, 0, 0];

    if sizeof_data == 0 {
        ctx.binary(void_data);
        println!("sent empty data");
        return;
    }

    let total_u32 = match u32::try_from(sizeof_data) {
        Ok(v) => v,
        Err(_) => {
            bin_err(ctx, "payload size exceeds u32::MAX");
            return;
        }
    };

    ctx.binary(total_u32.to_le_bytes().to_vec());
    let cfg = bincode::config::standard();
    let mut writer = WsExactChunkWriter::new(ctx, sizeof_data);

    if let Err(e) = bincode::encode_into_std_write(&data, &mut writer, cfg) {
        let _ = writer.flush();
        bin_err(writer.ctx, e);
        return;
    }

    if let Err(e) = writer.flush() {
        bin_err(writer.ctx, format!("flush error: {e}"));
        return;
    }

    debug_assert_eq!(writer.written_total, sizeof_data, "bincode size mismatch");
    println!(
        "sent data: {} bytes (header 4 + payload {})",
        4 + sizeof_data,
        sizeof_data
    );
}
