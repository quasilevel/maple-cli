use std::fmt;
use zbus::{
    blocking::{fdo, Connection},
    names::OwnedBusName,
};

pub struct RawPlayer {
    interface: OwnedBusName,
}

impl fmt::Display for RawPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.interface)
    }
}

impl super::Player for RawPlayer {
    fn find_all() -> Vec<Self> {
        let connection = Connection::session().unwrap();
        let names = fdo::DBusProxy::new(&connection)
            .unwrap()
            .list_names()
            .unwrap();

        names
            .into_iter()
            .map(|name| Self { interface: name })
            .collect::<Vec<_>>()
    }

    fn queue(&self, _url: String) -> super::Result<()> {
        // let connection = Connection::session()?;
        // let proxy = Proxy::new(
        //     &connection,
        //     self.interface,
        //     "/org/mpris/MediaPlayer2",
        //     "org.mpris.MediaPlayer2.TrackList",
        // )?;

        // proxy.call_noreply("TrackAdd", zbus::zvariant);

        // super::Result::Ok(())
        unimplemented!("queueing is not implemented in the player yet")
    }
}
