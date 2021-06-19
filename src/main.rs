use std::collections::HashMap;

use error::Error;
use session::SessionProxy;

mod error;
mod request;
mod response;
mod screencast;
mod session;

async fn create_session() -> Result<SessionProxy<'static>, Error> {
    let connection = zbus::azync::Connection::new_session().await?;
    let proxy = screencast::ScreenCastProxy::new(&connection).await?;
    let session = proxy.create_session(HashMap::new()).await?;
    Ok(session)
}

fn main() {
    println!("Hello, world!");
}
