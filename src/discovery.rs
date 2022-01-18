use anyhow::Result;
use crossbeam_channel::Receiver;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use reqwest::blocking;

use crate::Thing;

fn get_thing(info: ServiceInfo) -> Result<Thing> {
    let host = info.get_addresses().iter().next().unwrap();
    let port = info.get_port();

    let r = blocking::get(format!("http://{}:{}", host, port))?;

    let t = r.json()?;

    Ok(t)
}

/// Point of truth to access Things as consumer
pub struct Context {
    mdns: ServiceDaemon,
}

pub struct Iter {
    receiver: Receiver<ServiceEvent>,
}

impl Iter {
    fn from_receiver(receiver: Receiver<ServiceEvent>) -> Self {
        Self { receiver }
    }
}

impl Iterator for Iter {
    type Item = ServiceEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.recv().ok()
    }
}

impl Context {
    /// Creates a new Context composed by a series of Thing.
    pub fn new() -> Self {
        let mdns = ServiceDaemon::new().expect("Cannot run the daemon");
        Self { mdns }
    }
    /// Returns an Iterator over the discovered things
    pub fn things<'a>(&'a self) -> impl Iterator<Item = Result<Thing>> {
        let service_type = "_webthing._tcp.local.";
        let receiver = self.mdns.browse(service_type).expect("Failed to browse");

        Iter::from_receiver(receiver).filter_map(|v| match v {
            ServiceEvent::ServiceResolved(info) => Some(get_thing(info)),
            _ => None,
        })
    }
}
