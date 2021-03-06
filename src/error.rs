use crate::response::ResponseError;

#[derive(Debug)]
pub enum Error {
    Portal(ResponseError),
    ZbusFdo(zbus::fdo::Error),
    Zbus(zbus::Error),
    Zvariant(zvariant::Error),
    DBusMalformedMessage(zbus::MessageError),
    NoResponse,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Portal(e) => f.write_str(&format!("Portal response error: {}", e)),
            Self::ZbusFdo(e) => f.write_str(&format!("zbus fdo error: {}", e)),
            Self::DBusMalformedMessage(e) => {
                f.write_str(&format!("zbus malformed message error: {}", e))
            }
            Self::Zbus(e) => f.write_str(&format!("zbus error: {}", e)),
            Self::Zvariant(e) => f.write_str(&format!("zvariant error: {}", e)),
            Self::NoResponse => f.write_str("portal error: no response"),
        }
    }
}
impl From<ResponseError> for Error {
    fn from(e: ResponseError) -> Self {
        Self::Portal(e)
    }
}

impl From<zbus::MessageError> for Error {
    fn from(e: zbus::MessageError) -> Self {
        Self::DBusMalformedMessage(e)
    }
}

impl From<zbus::Error> for Error {
    fn from(e: zbus::Error) -> Self {
        Self::Zbus(e)
    }
}

impl From<zbus::fdo::Error> for Error {
    fn from(e: zbus::fdo::Error) -> Self {
        Self::ZbusFdo(e)
    }
}

impl From<zvariant::Error> for Error {
    fn from(e: zvariant::Error) -> Self {
        Self::Zvariant(e)
    }
}
