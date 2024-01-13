use relm4::Sender;
use std::time::Duration;
use tokio::{sync::oneshot, time};

use crate::Message;

use super::service::DisplayService;

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

pub struct DisplayServiceHandle {
    status: ServiceStatus,
}

impl DisplayServiceHandle {
    pub fn new() -> Self {
        Self {
            status: ServiceStatus::INACTIVE,
        }
    }

    pub async fn run(&mut self, sender: Sender<Message>) {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            match DisplayService::get_display_brightness().await {
                //if success send message to update display brightness or else send dummy message to update display brightness to 10
                Ok(brightness) => {
                    info!("Display brightness: {}", brightness);
                    sender
                        .send(Message::DisplayBrightnessChanged(brightness))
                        .unwrap();
                }
                Err(e) => {
                    error!("Error getting display brightness: {}", e);
                    sender.send(Message::DisplayBrightnessChanged(10)).unwrap();
                }
            };

            // match DisplayService::set_display_brightness(brightness_data).await {
            //     Ok(_) => {
            //      let _ = sender.send(Message::DisplayBrightnessChanged(brightness_data));
            //     }
            //     Err(e) => {
            //         error!("Error setting display brightness: {}", e);
            //     }
            // };
        }
    }

    pub fn stop(&mut self) {
        self.status = ServiceStatus::STOPPED;
    }

    pub fn start(&mut self) {
        self.status = ServiceStatus::STARTED;
    }
}
