
use self::client::{WDClient, WDClientError, body::WDBody};

use super::event::event_queue::WDEventQueue;


pub struct BasicWDApplication {
    base_url: String,
    name: String,
    client: WDClient
}

impl BasicWDApplication {
    pub async fn new(base_url: &str, name: &str) -> Result<Self, WDClientError> {
        let client = WDClient::new(base_url, name).await?;
        Ok(Self::with_client(base_url, name, client))
    }

    pub fn with_client(base_url: &str, name: &str, client: WDClient) -> Self {
        BasicWDApplication {
            base_url: base_url.to_owned(),
            name: name.to_owned(),
            client,
        }
    }

    pub fn event_queue(&mut self) -> &mut WDEventQueue {
        &mut self.client.event_queue
    }

    pub async fn send_event(&mut self) -> Result<(), WDClientError> {
        self.client.send_event(&self.base_url);
        Ok(())
    }

    pub fn body(&self) -> Result<WDBody, WDClientError> {
        Ok(self.client.body()?)
    }
}

pub mod client;