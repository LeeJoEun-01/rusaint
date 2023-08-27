use indexmap::IndexMap;

use crate::webdynpro::event::{WDEvent, WDEventBuilder, ucf_parameters::{UcfParametersBuilder, UcfResponseData, UcfAction}};

use super::Component;

pub struct Button<'a> {
    id: &'a str
}

impl<'a> Component<'a> for Button<'a> {}

impl<'a> Button<'a> {
    
    pub fn new(id: &'a str) -> Self {
        Self {
            id
        }
    }

    pub fn press(&self) -> WDEvent {
        let mut parameters: IndexMap<String, String> = IndexMap::new();
        let ucf_params = UcfParametersBuilder::default()
            .response(Some(UcfResponseData::Delta))
            .action(Some(UcfAction::Submit))
            .build()
            .unwrap();
        parameters.insert("Id".to_string(), self.id.clone().to_string());
        WDEventBuilder::default()
            .event("Press".to_owned())
            .control("Button".to_owned())
            .parameters(parameters)
            .ucf_parameters(ucf_params)
            .build()
            .unwrap()
    }
}