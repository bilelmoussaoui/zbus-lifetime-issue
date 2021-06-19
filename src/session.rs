use crate::error::Error;
use zvariant::OwnedObjectPath;
pub struct SessionProxy<'a>(zbus::azync::Proxy<'a>, zbus::azync::Connection);

impl<'a> SessionProxy<'a> {
    pub async fn new_owned(
        connection: zbus::azync::Connection,
        path: OwnedObjectPath,
    ) -> Result<SessionProxy<'a>, Error> {
        let proxy = zbus::ProxyBuilder::new_bare(&connection)
            .interface("org.freedesktop.portal.Session")
            .path(path)?
            .destination("org.freedesktop.portal.Desktop")
            .build_async()
            .await?;
        Ok(Self(proxy, connection.clone()))
    }
}
