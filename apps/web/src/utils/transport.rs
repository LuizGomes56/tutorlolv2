//! Realtime input decoding and Bincode transport for the HTTP calculation service.
#[cfg(any(test, not(all(feature = "tauri", feature = "local-calculation"))))]
use super::BINCODE_CONFIG;
use math::{
    model::{InputGame, OutputGame},
    riot::RiotRealtime,
};

#[cfg(not(feature = "local-calculation"))]
use crate::utils::api::WebClient;

#[cfg(not(feature = "local-calculation"))]
use ::api::{NoHeaders, NoPath, NoQuery};

pub type Error = Box<dyn core::error::Error>;

#[cfg(not(all(feature = "tauri", feature = "local-calculation")))]
fn consume_all(consumed: usize, length: usize) -> Result<(), Error> {
    if consumed != length {
        return Err("Unexpected trailing bytes in Bincode payload".into());
    }

    Ok(())
}

pub fn decode_realtime_input(bytes: &[u8]) -> Result<RiotRealtime<'_>, Error> {
    #[cfg(all(feature = "tauri", feature = "local-calculation"))]
    {
        Ok(serde_json::from_slice(bytes)?)
    }

    #[cfg(not(all(feature = "tauri", feature = "local-calculation")))]
    {
        let (input, consumed) = bincode::borrow_decode_from_slice(bytes, BINCODE_CONFIG)?;

        consume_all(consumed, bytes.len())?;

        Ok(input)
    }
}

pub async fn calculator(input: InputGame<'_>) -> Result<OutputGame, Error> {
    #[cfg(feature = "local-calculation")]
    {
        Ok(math::calculator::calculator(input))
    }

    #[cfg(not(feature = "local-calculation"))]
    {
        let response = WebClient::public()
            .send::<::api::Calculator>(&NoPath, &NoQuery, &NoHeaders, &input, None)
            .await?;
        Ok(response.decode()?)
    }
}

pub async fn realtime(
    #[cfg(not(feature = "tauri"))] bytes: &[u8],
    #[cfg(feature = "tauri")] bytes: web_sys::js_sys::Uint8Array,
) -> Result<crate::livegame::Game, Error> {
    #[cfg(feature = "local-calculation")]
    {
        // Borrowed decoding needs the Rust buffer to live through calculation.
        #[cfg(feature = "tauri")]
        let buffer = bytes.to_vec();

        #[cfg(feature = "tauri")]
        let bytes = buffer.as_slice();

        let input = decode_realtime_input(bytes)?;
        let output = math::realtime::realtime(&input).map_err(|error| error.to_string())?;

        Ok(output.into())
    }

    #[cfg(not(feature = "local-calculation"))]
    {
        #[cfg(feature = "tauri")]
        let buffer = bytes.to_vec();

        #[cfg(feature = "tauri")]
        let bytes = buffer.as_slice();
        let input = decode_realtime_input(bytes)?;

        let response = WebClient::public()
            .send::<::api::Realtime>(&NoPath, &NoQuery, &NoHeaders, &input, None)
            .await?;

        let output = response.decode()?;

        Ok(output.into())
    }
}

/// Calculate an already decoded viewer frame without serializing it locally.
pub async fn realtime_from(input: &RiotRealtime<'_>) -> Result<crate::livegame::Game, Error> {
    #[cfg(feature = "local-calculation")]
    {
        Ok(math::realtime::realtime(input)
            .map_err(|e| format!("Unable to calculate frame: {e:?}"))?
            .into())
    }

    #[cfg(not(feature = "local-calculation"))]
    {
        let response = WebClient::public()
            .send::<::api::Realtime>(&NoPath, &NoQuery, &NoHeaders, input, None)
            .await?;

        let output = response.decode()?;
        Ok(output.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(all(feature = "tauri", feature = "local-calculation")))]
    #[test]
    fn fixture_is_bincode_and_rejects_trailing_or_truncated_data() {
        let fixture = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/example.bincode"));
        assert!(decode_realtime_input(fixture).is_ok());
        let mut trailing = fixture.to_vec();
        trailing.push(0);
        assert!(decode_realtime_input(&trailing).is_err());
        assert!(decode_realtime_input(&fixture[..fixture.len() / 2]).is_err());
    }

    #[cfg(all(feature = "tauri", feature = "local-calculation"))]
    #[test]
    fn tauri_local_input_decodes_riot_json() {
        let fixture = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/example.bincode"));
        let (expected, _) =
            bincode::borrow_decode_from_slice::<RiotRealtime<'_>, _>(fixture, BINCODE_CONFIG)
                .unwrap();
        let json = serde_json::to_vec(&expected).unwrap();

        let input = decode_realtime_input(&json).expect("Tauri supplies Riot JSON bytes");
        assert_eq!(input, expected);

        let mut trailing = json.clone();
        trailing.push(b'!');
        assert!(decode_realtime_input(&trailing).is_err());
        assert!(decode_realtime_input(&json[..json.len() / 2]).is_err());
    }
}
