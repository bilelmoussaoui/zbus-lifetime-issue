use crate::error::Error;
use futures_lite::StreamExt;
use serde::de::DeserializeOwned;

use crate::response::Response;

pub struct RequestProxy<'a>(zbus::azync::Proxy<'a>, zbus::azync::Connection);

impl<'a> RequestProxy<'a> {
    pub async fn new_owned(
        connection: zbus::azync::Connection,
        path: zvariant::OwnedObjectPath,
    ) -> Result<RequestProxy<'a>, Error> {
        let proxy = zbus::azync::Proxy::new_owned(
            connection.clone(),
            "org.freedesktop.portal.Desktop".to_string(),
            path,
            "org.freedesktop.portal.Request".to_string(),
        )
        .await?;
        Ok(Self(proxy, connection))
    }

    pub async fn receive_response<R>(&self) -> Result<R, Error>
    where
        R: DeserializeOwned + zvariant::Type,
    {
        let mut stream = self.0.receive_signal("Response").await?;
        let message = stream.next().await.ok_or(Error::NoResponse)?;
        let body = message.body::<Response<R>>()?;
        match body {
            Response::Err(e) => Err(e.into()),
            Response::Ok(r) => Ok(r),
        }
    }
}
