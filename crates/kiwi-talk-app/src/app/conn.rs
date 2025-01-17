use talk_loco_client::{
    client::{booking::BookingClient, checkin::CheckinClient},
    LocoRequestSession,
};
use talk_loco_command::{
    request::{booking::GetConfReq, checkin::CheckinReq},
    response::{booking::GetConfRes, checkin::CheckinRes},
    structs::client::ClientInfo,
};
use thiserror::Error;
use tokio_native_tls::native_tls;

use crate::error::impl_tauri_error;

use super::{
    constants::{
        BOOKING_SERVER, CHECKIN_SERVER, TALK_MCCMNC, TALK_MODEL, TALK_NET_TYPE, TALK_OS,
        TALK_USE_SUB, TALK_VERSION,
    },
    stream::{create_secure_stream, create_tls_stream, LOCO_CLIENT_SECURE_SESSION},
};

pub async fn get_conf() -> Result<GetConfRes, ConnError> {
    let mut connector = tokio_native_tls::TlsConnector::from(
        native_tls::TlsConnector::new().or(Err(ConnError::Connection))?,
    );

    let stream = create_tls_stream(&mut connector, BOOKING_SERVER.0, BOOKING_SERVER)
        .await
        .or(Err(ConnError::Connection))?;

    let (session, _) = LocoRequestSession::new(stream);
    let client = BookingClient(&session);

    client
        .get_conf(&GetConfReq {
            os: TALK_OS.into(),
            mccmnc: TALK_MCCMNC.into(),
            model: TALK_MODEL.into(),
        })
        .await
        .or(Err(ConnError::Stream))
}

pub async fn checkin(user_id: i64) -> Result<CheckinRes, ConnError> {
    let stream = create_secure_stream(&LOCO_CLIENT_SECURE_SESSION, CHECKIN_SERVER)
        .await
        .or(Err(ConnError::Connection))?;

    let (session, _) = LocoRequestSession::new(stream);
    let client = CheckinClient(&session);

    client
        .checkin(&CheckinReq {
            user_id,
            client: ClientInfo {
                os: TALK_OS.into(),
                net_type: TALK_NET_TYPE,
                app_version: TALK_VERSION.into(),
                mccmnc: TALK_MCCMNC.into(),
            },
            language: "ko".into(),
            country_iso: "KR".into(),
            use_sub: TALK_USE_SUB,
        })
        .await
        .or(Err(ConnError::Stream))
}

#[derive(Debug, Error)]
pub enum ConnError {
    #[error("cannot connect to server")]
    Connection,

    #[error("stream error")]
    Stream,

    #[error("request failed. status: {0}")]
    Request(i16),
}

impl_tauri_error!(ConnError);
