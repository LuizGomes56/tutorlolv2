use crate::AppState;
use actix_web::{
    HttpResponse, get,
    http::header::{CONTENT_LENGTH, HeaderName},
    post,
    web::{Bytes, Data},
};
use bincode::{Encode, config::Configuration};
use rand::random_range;
use tutorlolv2_math::{
    math::{calculator::calculator, realtime::realtime},
    model::Sizer,
};
use uuid::Uuid;

type Response = Result<HttpResponse, Box<dyn std::error::Error>>;

const BINCODE_CONFIG: Configuration = bincode::config::standard();
const OCTET_STREAM: (HeaderName, &'static str) =
    (crate::header::CONTENT_TYPE, "application/octet-stream");

fn respond(data: impl Sizer + Encode) -> Response {
    unsafe {
        let size = data.size();
        let mut buf = Box::<[u8]>::new_uninit_slice(size);
        let raw = std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, size);
        bincode::encode_into_slice(&data, raw, BINCODE_CONFIG)?;
        let init = buf.assume_init();
        Ok(HttpResponse::Ok()
            .insert_header(OCTET_STREAM)
            .body(Bytes::from(init)))
    }
}

#[post("/realtime")]
pub async fn realtime_handler(body: Bytes) -> Response {
    let game_data = serde_json::from_slice(&body)?;
    let data = realtime(&game_data)?;
    respond(data)
}

#[post("/calculator")]
pub async fn calculator_handler(body: Bytes) -> Response {
    let (decoded, _) = bincode::decode_from_slice(&body, BINCODE_CONFIG)?;
    let data = calculator(decoded)?;
    respond(data)
}

const CODE_LENGTH: usize = 6;
const BODY_LENGTH: usize = 16 + CODE_LENGTH;
static CHARS: &'static [u8; 62] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

#[get("/create")]
pub async fn create_game_handler(state: Data<AppState>) -> Response {
    let mut game_code_bytes = [0; CODE_LENGTH];

    unsafe {
        for i in 0..CODE_LENGTH {
            *game_code_bytes.get_unchecked_mut(i) = CHARS[random_range(0..CHARS.len())];
        }

        let game_code = std::str::from_utf8_unchecked(&game_code_bytes);
        let uuid_v4 = Uuid::new_v4();
        let game_id = uuid_v4.into_bytes();

        let mut buffer = [0; BODY_LENGTH];
        buffer.get_unchecked_mut(..16).copy_from_slice(&game_id);
        buffer
            .get_unchecked_mut(16..)
            .copy_from_slice(&game_code_bytes.get_unchecked(..));

        sqlx::query("INSERT INTO games (game_id, game_code) VALUES ($1, $2)")
            .bind(&game_id)
            .bind(game_code)
            .execute(&state.db)
            .await?;

        Ok(HttpResponse::Ok()
            .insert_header(OCTET_STREAM)
            .insert_header((CONTENT_LENGTH, BODY_LENGTH))
            .body(Bytes::from(Box::<[u8]>::from(buffer))))
    }
}

#[post("/get_by_code")]
pub async fn get_by_code_handler(state: Data<AppState>, body: actix_web::web::Bytes) -> Response {
    let data = sqlx::query_as::<_, (String,)>(
        "SELECT g.game_id, gd.game_data AS game 
        FROM games g
        JOIN game_data gd ON gd.game_id = g.game_id 
        WHERE g.game_code = $1
        ORDER BY gd.game_time DESC 
        LIMIT 1",
    )
    .bind(body.as_ref())
    .fetch_one(&state.db)
    .await?;

    let riot_realtime = serde_json::from_str(&data.0)?;
    let realtime_data = realtime(&riot_realtime)?;
    respond(realtime_data)
}
