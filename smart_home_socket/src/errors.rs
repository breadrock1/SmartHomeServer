use thiserror::Error;

#[derive(Debug, Error)]
pub enum SocketError {
    #[error("Failed while switching socket")]
    SwitchingError(String),
    #[error("Failed while appending socket")]
    AppendingError(String),
    #[error("Failed while removing socket")]
    RemovingError(String),
    #[error("Failed while getting socket status")]
    StatusError(String),
    #[error("Failed while setting socket power")]
    PowerError(String),
}

pub type SocketResult = Result<String, SocketError>;

#[derive(Debug, Error)]
pub enum ThermometerError {
    #[error("Failed while trying update status")]
    UpdateError(String),
    #[error("Failed while trying setup status")]
    SetupError(String),
}

pub type ThermometerResult = Result<String, ThermometerError>;
pub type UdpResult = Result<(), ThermometerError>;
