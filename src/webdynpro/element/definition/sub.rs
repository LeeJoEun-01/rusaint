use std::borrow::Cow;

use scraper::Selector;

use crate::webdynpro::{
    client::body::Body,
    element::{Element, SubElement},
    error::{ElementError, WebDynproError},
};

use super::ElementDef;

/// [`SapTable`]등에서 사용하는 [`SubElement`]
#[derive(Debug)]
pub struct SubElementDef<'a, Parent, T>
where
    Parent: Element<'a>,
    T: SubElement<'a>,
{
    id: Cow<'static, str>,
    parent: ElementDef<'a, Parent>,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, Parent: Element<'a>, T: SubElement<'a>> Clone for SubElementDef<'a, Parent, T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            parent: self.parent.clone(),
            _marker: self._marker.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

impl<'a, Parent, T> SubElementDef<'a, Parent, T>
where
    Parent: Element<'a>,
    T: SubElement<'a>,
{
    /// 새로운 서브 엘리먼트의 정의를 만듭니다.
    pub const fn new(
        parent: ElementDef<'a, Parent>,
        id: &'static str,
    ) -> SubElementDef<'a, Parent, T> {
        SubElementDef {
            id: Cow::Borrowed(id),
            parent,
            _marker: std::marker::PhantomData,
        }
    }

    /// 런타임에서 서브 엘리먼트의 정의를 만듭니다.
    pub fn new_dynamic(parent: ElementDef<'a, Parent>, id: String) -> SubElementDef<'a, Parent, T> {
        SubElementDef {
            id: id.into(),
            parent,
            _marker: std::marker::PhantomData,
        }
    }

    /// 서브 엘리먼트의 Id를 반환합니다.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 서브 엘리먼트의 CSS Selector를 반환합니다.
    pub fn selector(&self) -> Result<Selector, WebDynproError> {
        Selector::parse(format!(r#"[id="{}"] [id="{}"]"#, self.parent.id, self.id).as_str()).or(
            Err(ElementError::InvalidId(format!(
                "{}, {}",
                self.parent.id, self.id
            )))?,
        )
    }

    /// [`Body`]에서 서브 엘리먼트를 가져옵니다.
    pub fn from_body(self, body: &'a Body) -> Result<T, WebDynproError> {
        T::from_body(self, body)
    }

    /// [`scraper::ElementRef`]에서 서브 엘리먼트를 가져옵니다.
    pub fn from_elem(self, element: scraper::ElementRef<'a>) -> Result<T, WebDynproError> {
        T::from_elem(self, element)
    }
}
