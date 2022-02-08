use std::time::{Duration, Instant};

use anyhow::Result;
use mdns_sd::{Receiver, ServiceDaemon, ServiceEvent, ServiceInfo};
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
pub struct Discovery {
    mdns: ServiceDaemon,
    service_type: String,
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

pub struct IterTimeout {
    receiver: Receiver<ServiceEvent>,
    timeout: Duration,
}

impl IterTimeout {
    fn from_receiver(receiver: Receiver<ServiceEvent>, timeout: Duration) -> Self {
        Self { receiver, timeout }
    }
}

impl Iterator for IterTimeout {
    type Item = ServiceEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.recv_timeout(self.timeout).ok()
    }
}

impl Default for Discovery {
    fn default() -> Self {
        Self::new()
    }
}

impl Discovery {
    /// Creates a new Context composed by a series of Thing.
    pub fn new() -> Self {
        let mdns = ServiceDaemon::new().expect("Cannot run the daemon");
        let service_type = "_webthing._tcp.local.".to_owned();
        Self { mdns, service_type }
    }
    /// Returns an Iterator over the discovered things
    pub fn things(&self) -> impl Iterator<Item = Result<Thing>> {
        let receiver = self
            .mdns
            .browse(&self.service_type)
            .expect("Failed to browse");

        Iter::from_receiver(receiver).filter_map(|v| match v {
            ServiceEvent::ServiceResolved(info) => Some(get_thing(info)),
            _ => None,
        })
    }
    /// Discovers things and interrupts the search after a certain time.
    pub fn discover_timeout(&self, timeout: Duration) -> Result<Vec<Thing>> {
        let receiver = self
            .mdns
            .browse(&self.service_type)
            .expect("Failed to browse");

        let now = Instant::now();

        IterTimeout::from_receiver(receiver, Duration::from_secs(1))
            .take_while(|_| now.elapsed() > timeout)
            .filter_map(|v| match v {
                ServiceEvent::ServiceResolved(info) => Some(get_thing(info)),
                _ => None,
            })
            .collect()
    }
}
