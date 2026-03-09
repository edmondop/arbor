use std::sync::mpsc;
use std::thread;

use zeroconf::prelude::*;
use zeroconf::{BrowserEvent, MdnsBrowser, ServiceType};

const SERVICE_NAME: &str = "arbor";
const SERVICE_PROTOCOL: &str = "tcp";

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoveredDaemon {
    pub instance_name: String,
    pub host: String,
    pub addresses: Vec<String>,
    pub port: u16,
    pub tls: bool,
    pub has_auth: bool,
    pub version: String,
}

impl DiscoveredDaemon {
    pub fn base_url(&self) -> String {
        let scheme = if self.tls { "https" } else { "http" };
        let host = self
            .addresses
            .first()
            .cloned()
            .unwrap_or_else(|| self.host.clone());
        format!("{scheme}://{host}:{}", self.port)
    }

    /// Short display label — hostname without `.local.` suffix.
    pub fn display_name(&self) -> &str {
        self.instance_name
            .strip_suffix(".local.")
            .unwrap_or(&self.instance_name)
    }
}

pub enum MdnsEvent {
    Added(DiscoveredDaemon),
    Removed(String),
}

pub trait MdnsDiscovery {
    fn poll_updates(&self) -> Vec<MdnsEvent>;
}

struct ZeroconfBrowser {
    receiver: mpsc::Receiver<MdnsEvent>,
    _handle: thread::JoinHandle<()>,
}

/// Start browsing for `_arbor._tcp` services on the local network.
pub fn start_browsing() -> Result<Box<dyn MdnsDiscovery>, String> {
    let service_type = ServiceType::new(SERVICE_NAME, SERVICE_PROTOCOL)
        .map_err(|e| format!("failed to create service type: {e}"))?;

    let (tx, rx) = mpsc::channel();

    let handle = thread::Builder::new()
        .name("mdns-browse".into())
        .spawn(move || {
            let mut browser = MdnsBrowser::new(service_type);

            browser.set_service_callback(Box::new(move |result, _| {
                match result {
                    Ok(BrowserEvent::Add(service)) => {
                        let tls = service
                            .txt()
                            .as_ref()
                            .and_then(|t| t.get("tls"))
                            .is_some_and(|v| v == "true");
                        let has_auth = service
                            .txt()
                            .as_ref()
                            .and_then(|t| t.get("auth"))
                            .is_some_and(|v| v == "true");
                        let version = service
                            .txt()
                            .as_ref()
                            .and_then(|t| t.get("version"))
                            .unwrap_or_default();

                        let daemon = DiscoveredDaemon {
                            instance_name: service.name().to_owned(),
                            host: service.host_name().to_owned(),
                            addresses: vec![service.address().to_owned()],
                            port: *service.port(),
                            tls,
                            has_auth,
                            version,
                        };

                        let _ = tx.send(MdnsEvent::Added(daemon));
                    }
                    Ok(BrowserEvent::Remove(removal)) => {
                        let _ = tx.send(MdnsEvent::Removed(removal.name().to_owned()));
                    }
                    Err(err) => {
                        tracing::warn!("mDNS browse error: {err}");
                    }
                }
            }));

            let event_loop = match browser.browse_services() {
                Ok(el) => el,
                Err(err) => {
                    tracing::error!("mDNS browse_services failed: {err}");
                    return;
                }
            };

            loop {
                if let Err(err) = event_loop.poll(std::time::Duration::from_secs(1)) {
                    tracing::warn!("mDNS browse event loop error: {err}");
                    break;
                }
            }
        })
        .map_err(|e| format!("failed to spawn mDNS browse thread: {e}"))?;

    Ok(Box::new(ZeroconfBrowser {
        receiver: rx,
        _handle: handle,
    }))
}

impl MdnsDiscovery for ZeroconfBrowser {
    /// Non-blocking drain of pending mDNS events.
    fn poll_updates(&self) -> Vec<MdnsEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.receiver.try_recv() {
            events.push(event);
        }
        events
    }
}
