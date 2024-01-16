//use device_oem_info_service

use relm4::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use crate::Message;

use super::service::DeviceOEMInfo;

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

pub struct DeviceOEMInfoHandle {
    status: ServiceStatus,
}

impl DeviceOEMInfoHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            match DeviceOEMInfo::get_device_oem_info_service() {
                //if success send message to update display brightness or else send dummy message to update display brightness to 10
                Ok(distro_info) => {
                    info!("Distro info: {:?}", distro_info);
                    sender
                        .send(Message::OsNameChanged(distro_info.name.to_string().trim().to_string()))
                        .unwrap();
                    sender.send(Message::OsVersionChanged(distro_info.version.to_string().trim().to_string())).unwrap();
                }
                Err(e) => {
                    error!("Error getting device oem info: {}", e);
                    // sender.send(Message::DeviceOEMInfoChanged(DistroInfo::new())).unwrap();
                }
            };

        }
    }
}
