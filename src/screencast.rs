use crate::error::Error;
use crate::request::RequestProxy;
use crate::session::SessionProxy;
use std::collections::HashMap;
use std::convert::TryFrom;
use zvariant::Value;
use zvariant_derive::{DeserializeDict, SerializeDict, TypeDict};

#[derive(SerializeDict, DeserializeDict, TypeDict, Debug)]
/// A response to the create session request.
struct CreateSession {
    /// A string that will be used as the last element of the session handle.
    // TODO: investigate why this doesn't return an ObjectPath
    pub(crate) session_handle: String,
}

pub struct ScreenCastProxy<'a>(zbus::azync::Proxy<'a>);

impl<'a> ScreenCastProxy<'a> {
    pub async fn new(connection: &zbus::azync::Connection) -> Result<ScreenCastProxy<'a>, Error> {
        let proxy = zbus::ProxyBuilder::new_bare(connection)
            .interface("org.freedesktop.portal.ScreenCast")
            .path("/org/freedesktop/portal/desktop")?
            .destination("org.freedesktop.portal.Desktop")
            .build_async()
            .await?;
        Ok(Self(proxy))
    }

    /// Create a screen cast session.
    pub async fn create_session(
        &self,
        options: HashMap<&str, Value<'_>>,
    ) -> Result<SessionProxy<'_>, Error> {
        let path: zvariant::OwnedObjectPath = self
            .0
            .call_method("CreateSession", &(options))
            .await?
            .body()?;
        let request = RequestProxy::new_owned(self.0.connection().clone(), path).await?;
        let session = request.receive_response::<CreateSession>().await?;
        SessionProxy::new_owned(
            self.0.connection().clone(),
            zvariant::OwnedObjectPath::try_from(session.session_handle)?,
        )
        .await
    }
}
