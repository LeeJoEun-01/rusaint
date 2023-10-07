use std::{borrow::Cow, cell::OnceCell};

use serde_json::Value;

use crate::webdynpro::error::WebDynproError;

use super::{Element, ElementDef, EventParameterMap, Interactable};

// Type for unimplemented elements
#[derive(Debug)]
pub struct Unknown<'a> {
    id: Cow<'static, str>,
    element_ref: scraper::ElementRef<'a>,
    ct: OnceCell<Option<String>>,
    lsdata: OnceCell<Option<Value>>,
    lsevents: OnceCell<Option<EventParameterMap>>,
}

impl<'a> Element<'a> for Unknown<'a> {
    const CONTROL_ID: &'static str = "_UNKNOWN";

    const ELEMENT_NAME: &'static str = "Unknown";

    type ElementLSData = Value;

    fn lsdata(&self) -> Option<&Self::ElementLSData> {
        self.lsdata
            .get_or_init(|| {
                let lsdata_obj = Self::lsdata_elem(self.element_ref).ok()?;
                serde_json::from_value::<Self::ElementLSData>(lsdata_obj).ok()
            })
            .as_ref()
    }

    fn from_elem(
        elem_def: ElementDef<'a, Self>,
        element: scraper::ElementRef<'a>,
    ) -> Result<Self, WebDynproError> {
        Ok(Self::new(elem_def.id.to_owned(), element))
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn element_ref(&self) -> &scraper::ElementRef<'a> {
        &self.element_ref
    }

    fn wrap(self) -> super::ElementWrapper<'a> {
        super::ElementWrapper::Unknown(self)
    }

    fn children(&self) -> Vec<super::ElementWrapper<'a>> {
        Self::children_elem(self.element_ref().clone())
    }
}

impl<'a> Interactable<'a> for Unknown<'a> {
    fn lsevents(&self) -> Option<&EventParameterMap> {
        self.lsevents
            .get_or_init(|| Self::lsevents_elem(self.element_ref).ok())
            .as_ref()
    }
}

impl<'a> Unknown<'a> {
    pub fn new(id: Cow<'static, str>, element_ref: scraper::ElementRef<'a>) -> Self {
        Self {
            id,
            element_ref,
            ct: OnceCell::new(),
            lsdata: OnceCell::new(),
            lsevents: OnceCell::new(),
        }
    }

    pub fn ct(&self) -> Option<&String> {
        self.ct
            .get_or_init(|| {
                self.element_ref
                    .value()
                    .attr("ct")
                    .and_then(|str| Some(str.to_string()))
            })
            .as_ref()
    }
}