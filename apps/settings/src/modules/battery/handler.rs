use relm4::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use crate::{Message,};

use super::service::BatteryService;

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

pub struct BatteryServiceHandle {
    status: ServiceStatus,
}

impl BatteryServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            match BatteryService::get_battery_status().await {
                //if success send message to update battery percentage or else send dummy message to update battery percentage to 10
                Ok(percentage) => {
                    info!("Battery percentage: {}", percentage);
                    sender.send(Message::BatteryPercentageChanged(percentage)).unwrap();
                }
                Err(e) => {
                    error!("Error getting battery percentage: {}", e);
                    sender.send(Message::BatteryPercentageChanged(10)).unwrap();
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
