use derive_builder::Builder;

use super::{EVENT_DATA_START, EVENT_DATA_COLON, EVENT_DATA_COMMA, EVENT_DATA_END};

#[derive(Builder, Default, Clone)]
#[builder(default)]
pub struct UcfParameters {
    action: Option<UcfAction>,
    cardinality: Option<UcfCardinality>,
    transport: Option<UcfTransportMethod>,
    response: Option<UcfResponseData>,
    action_url: Option<String>,
    prepare_script: Option<String>,
    delay: Option<UcfDelay>,
    sync_execution: Option<bool>,
    client_listener: Option<String>
}

impl ToString for UcfParameters {
    fn to_string(&self) -> String {
        let mut owned = "".to_owned();
        owned.push_str(EVENT_DATA_START);
        if let Some(action) = &self.action {
            owned.push_str("ClientAction");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(&action.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(cardinality) = &self.cardinality {
            owned.push_str("EnqueueCardinality");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(&cardinality.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(transport) = &self.transport {
            owned.push_str("TransportMethod");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(&transport.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(response) = &self.response {
            owned.push_str("ResponseData");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(&response.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(action_url) = &self.action_url {
            owned.push_str("ActionUrl");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(action_url);
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(prepare_script) = &self.prepare_script {
            owned.push_str("PrepareScript");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(prepare_script);
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(delay) = &self.delay {
            owned.push_str("Delay");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(&delay.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(sync_execution) = &self.sync_execution {
            owned.push_str("SyncExecution");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(&sync_execution.to_string());
            owned.push_str(EVENT_DATA_COMMA);
        }
        if let Some(client_listener) = &self.client_listener {
            owned.push_str("ClientListener");
            owned.push_str(EVENT_DATA_COLON);
            owned.push_str(client_listener);
            owned.push_str(EVENT_DATA_COMMA);
        }
        if owned.ends_with(EVENT_DATA_COMMA) { owned.truncate(owned.len() - 5) };
        owned.push_str(EVENT_DATA_END);
        owned
    }
}

// TODO: Cleanup code
impl UcfParameters {
    pub fn serialize(&self) -> String {
        self.to_string()
    }
}

#[derive(Clone)]
pub enum UcfAction {
    Submit,
    SubmitAsync,
    Enqueue,
    None
}

impl ToString for UcfAction {
    fn to_string(&self) -> String {
        match &self {
            Self::Submit => "submit",
            Self::SubmitAsync => "submit_async",
            Self::Enqueue => "enqueue",
            _ => "none"
        }.to_owned()
    }
}

#[derive(Clone)]
pub enum UcfCardinality {
    Multiple,
    Single,
    None
}

impl ToString for UcfCardinality {
    fn to_string(&self) -> String {
        match &self {
            Self::Multiple => "multiple",
            Self::Single => "single",
            _ => "none"
        }.to_owned()
    }
}

#[derive(Clone)]
pub enum UcfResponseData {
    Full,
    Delta,
    Inherit
}

impl ToString for UcfResponseData {
    fn to_string(&self) -> String {
        match &self {
            Self::Full => "full",
            Self::Delta => "delta",
            Self::Inherit => "inherit"
        }.to_owned()
    }
}

#[derive(Clone)]
pub enum UcfTransportMethod {
    Full,
    Partial
}

impl ToString for UcfTransportMethod {
    fn to_string(&self) -> String {
        match &self {
            Self::Full => "full",
            Self::Partial => "partial"
        }.to_owned()
    }
}

#[derive(Clone)]
pub enum UcfDelay {
    Full,
    None
}

impl ToString for UcfDelay {
    fn to_string(&self) -> String {
        match &self {
            Self::Full => "full",
            _ => "none"
        }.to_owned()
    }
}