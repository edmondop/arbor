use std::sync::Arc;
use std::thread;

use zeroconf::prelude::*;
use zeroconf::{MdnsService, ServiceType, TxtRecord};

const SERVICE_NAME: &str = "arbor";
const SERVICE_PROTOCOL: &str = "tcp";

#[derive(Debug, thiserror::Error)]
pub enum MdnsError {
    #[error("failed to create mDNS service: {0}")]
    ServiceInit(zeroconf::error::Error),
    #[error("failed to register mDNS service: {0}")]
    Registration(String),
}

/// Holds the mDNS registration thread alive. The service is unregistered when dropped
/// (the thread exits when the shutdown flag is set, or when the process ends).
pub struct MdnsRegistration {
    _handle: thread::JoinHandle<()>,
}

/// Register this arbor-httpd instance on the local network via DNS-SD / Bonjour.
pub fn register_service(
    port: u16,
    tls: bool,
    has_auth: bool,
) -> Result<MdnsRegistration, MdnsError> {
    let service_type =
        ServiceType::new(SERVICE_NAME, SERVICE_PROTOCOL).map_err(MdnsError::ServiceInit)?;

    let mut txt = TxtRecord::new();
    let _ = txt.insert("tls", if tls { "true" } else { "false" });
    let _ = txt.insert("auth", if has_auth { "true" } else { "false" });
    let _ = txt.insert("version", env!("CARGO_PKG_VERSION"));

    let instance_name = hostname::get()
        .ok()
        .and_then(|h: std::ffi::OsString| h.into_string().ok())
        .unwrap_or_else(|| "arbor-httpd".to_owned());

    let shutdown = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let shutdown_clone = Arc::clone(&shutdown);

    let handle = thread::Builder::new()
        .name("mdns-register".into())
        .spawn(move || {
            let mut service = MdnsService::new(service_type, port);
            service.set_name(&instance_name);
            service.set_txt_record(txt);
            service.set_registered_callback(Box::new(|result, _| {
                match result {
                    Ok(registration) => {
                        tracing::info!("mDNS service registered: {}", registration.name());
                    }
                    Err(err) => {
                        tracing::error!("mDNS registration error: {err}");
                    }
                }
            }));

            let event_loop = match service.register() {
                Ok(el) => el,
                Err(e) => {
                    tracing::error!("mDNS register failed: {e}");
                    return;
                }
            };

            loop {
                if shutdown_clone.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
                if let Err(err) = event_loop.poll(std::time::Duration::from_secs(1)) {
                    tracing::warn!("mDNS event loop error: {err}");
                    break;
                }
            }
        })
        .map_err(|e| MdnsError::Registration(e.to_string()))?;

    Ok(MdnsRegistration { _handle: handle })
}
