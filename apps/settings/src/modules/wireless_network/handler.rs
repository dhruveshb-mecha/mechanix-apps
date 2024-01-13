use relm4::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use crate::Message;
use wifi_ctrl::sta::ScanResult;

use super::service::NetworkService;

use tracing::{error, info};

#[derive(Debug)]
pub enum ServiceMessage {
    Start { respond_to: oneshot::Sender<u32> },
    Stop { respond_to: oneshot::Sender<u32> },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ServiceStatus {
    INACTIVE = 0,
    STARTED = 1,
    STOPPED = -1,
}

pub struct NetworkServiceHandle {
    status: ServiceStatus,
}

impl NetworkServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            match NetworkService::get_network_list().await {
                //if success send message to update network status or else send dummy message to update network status
                Ok(status) => {
                
                    sender.send(Message::NetworkStatusChanged(status)).unwrap();
                }
                Err(e) => {
               
                    error!("Error getting wireless networks {}", e);

                    let scan_results = vec![
                        ScanResult {
                            mac: "mac1".to_string(),
                            frequency: "frequency1".to_string(),
                            flags: "flags1".to_string(),
                            name: "name1".to_string(),
                            signal: 2,
                        },
                        ScanResult {
                            mac: "mac2".to_string(),
                            frequency: "frequency2".to_string(),
                            flags: "flags2".to_string(),
                            name: "name2".to_string(),
                            signal: 3,
                        },
                    ];

                    sender
                        .send(Message::NetworkStatusChanged(scan_results))
                        .unwrap();
                }
            };
        }
    }

    pub fn stop(&mut self) {
        self.status = ServiceStatus::STOPPED;
    }

    pub fn start(&mut self) {
        self.status = ServiceStatus::STARTED;
    }
}
