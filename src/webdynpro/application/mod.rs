use self::client::{body::Body, Client};
use anyhow::Result;
use url::Url;

use super::{
    element::{form::Form, ElementDef},
    error::ClientError,
    event::Event,
};

pub struct BasicApplication {
    base_url: Url,
    name: String,
    client: Client,
}

impl BasicApplication {
    pub const SSR_FORM: ElementDef<Form> = ElementDef::new("sap.client.SsrClient.form");

    pub async fn new(base_url_str: &str, name: &str) -> Result<Self> {
        let base_url = Url::parse(base_url_str)?;
        let client = Client::new(&base_url, name).await?;
        Ok(Self::with_client(base_url, name, client)?)
    }

    pub fn with_client(base_url: Url, name: &str, client: Client) -> Result<Self> {
        Ok(BasicApplication {
            base_url,
            name: name.to_owned(),
            client,
        })
    }

    pub fn client_url(&self) -> String {
        let mut url = "".to_owned();
        url.push_str(&self.base_url.as_str());
        if !url.ends_with('/') {
            url.push_str("/");
        }
        url.push_str(&self.name);
        url.push_str("?sap-wd-stableids=X#");
        url
    }

    pub async fn send_events(&mut self, events: Vec<Event>) -> Result<()> {
        let client = &mut self.client;
        let form = Self::SSR_FORM.from_body(&client.body)?;
        for event in events.into_iter() {
            if !event.is_enqueable() && event.is_submitable() {
                client.add_event(event);
                client.add_event(
                    form.request(false, "", "", false, false)
                        .or(Err(ClientError::NoForm))?,
                );
                client.send_event(&self.base_url).await?;
            } else {
                client.add_event(event);
            }
        }
        Ok(())
    }

    pub fn body(&self) -> &Body {
        &self.client.body
    }

    pub fn body_mut(&mut self) -> &mut Body {
        &mut self.client.body
    }
}

pub mod client;
