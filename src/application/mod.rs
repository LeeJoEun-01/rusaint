use std::sync::Arc;
use url::Url;

use crate::{
    session::USaintSession,
    utils::DEFAULT_USER_AGENT,
    webdynpro::{
        client::{body::Body, EventProcessResult, WebDynproClient, WebDynproClientBuilder},
        command::WebDynproCommand,
        element::{
            define_elements,
            definition::ElementDefinition,
            system::{ClientInspector, Custom, CustomClientInfo, LoadingPlaceholder},
        },
        error::WebDynproError,
        event::Event,
    },
    RusaintError,
};

const SSU_WEBDYNPRO_BASE_URL: &str = "https://ecc.ssu.ac.kr/sap/bc/webdynpro/SAP/";
const INITIAL_CLIENT_DATA_WD01: &str = "ClientWidth:1920px;ClientHeight:1000px;ScreenWidth:1920px;ScreenHeight:1080px;ScreenOrientation:landscape;ThemedTableRowHeight:33px;ThemedFormLayoutRowHeight:32px;ThemedSvgLibUrls:{\"SAPGUI-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPGUI-icons.svg\",\"SAPWeb-icons\":\"https://ecc.ssu.ac.kr:8443/sap/public/bc/ur/nw5/themes/~cache-20210223121230/Base/baseLib/sap_fiori_3/svg/libs/SAPWeb-icons.svg\"};ThemeTags:Fiori_3,Touch;ThemeID:sap_fiori_3;SapThemeID:sap_fiori_3;DeviceType:DESKTOP";
const INITIAL_CLIENT_DATA_WD02: &str = "ThemedTableRowHeight:25px";
/// u-saint에 접속하기 위한 기본 클라이언트
pub struct USaintClient(WebDynproClient);

impl<'a> USaintClient {
    define_elements! {
        CLIENT_INSPECTOR_WD01: ClientInspector<'a> = "WD01";
        CLIENT_INSPECTOR_WD02: ClientInspector<'a> = "WD02";
        LOADING_PLACEHOLDER: LoadingPlaceholder<'a> = "_loadingPlaceholder_";
    }

    const CUSTOM: Custom = Custom::new(std::borrow::Cow::Borrowed("WD01"));

    async fn new(client: WebDynproClient) -> Result<USaintClient, WebDynproError> {
        let mut client = USaintClient(client);
        client.load_placeholder().await?;
        Ok(client)
    }

    /// WebDynpro 애플리케이션의 이름을 반환합니다.
    pub fn name(&self) -> &str {
        self.0.name()
    }

    /// WebDynpro 애플리케이션의 기본 URL을 반환합니다.
    pub fn base_url(&self) -> &Url {
        self.0.base_url()
    }

    /// WebDynpro 애플리케이션의 페이지 문서를 반환합니다.
    pub fn body(&self) -> &Body {
        self.0.body()
    }

    /// 실제로 요청하는 애플리케이션의 URL을 반환합니다.
    pub fn client_url(&self) -> String {
        self.0.client_url()
    }

    /// WebDynpro 클라이언트에 명령을 전송합니다.
    pub async fn send<T: WebDynproCommand>(
        &mut self,
        command: T,
    ) -> Result<T::Result, WebDynproError> {
        self.0.send(command).await
    }

    /// 이벤트를 처리합니다. [`process_event()`](crate::webdynpro::client::WebDynproClient::process_event)를 참조하세요.
    pub async fn process_event(
        &mut self,
        force_send: bool,
        event: Event,
    ) -> Result<EventProcessResult, WebDynproError> {
        self.0.process_event(force_send, event).await
    }

    async fn load_placeholder(&mut self) -> Result<(), WebDynproError> {
        let events = {
            let body = self.body();
            let wd01 = Self::CLIENT_INSPECTOR_WD01.from_body(body)?;
            let wd02 = Self::CLIENT_INSPECTOR_WD02.from_body(body)?;
            let load_ph = Self::LOADING_PLACEHOLDER.from_body(body)?;
            let client_infos = Self::CUSTOM.client_infos(CustomClientInfo {
                client_url: self.client_url(),
                document_domain: "ssu.ac.kr".to_owned(),
                ..CustomClientInfo::default()
            });
            vec![
                wd01.notify(INITIAL_CLIENT_DATA_WD01)?,
                wd02.notify(INITIAL_CLIENT_DATA_WD02)?,
                load_ph.load()?,
                client_infos,
            ]
        };
        for event in events {
            self.process_event(false, event).await?;
        }
        Ok(())
    }
}

/// U-Saint 애플리케이션이 구현하는 트레이트
pub trait USaintApplication: Sized {
    /// U-Saint WebDynpro 애플리케이션 이름
    const APP_NAME: &'static str;

    /// U-Saint 클라이언트를 애플리케이션으로 변환합니다.
    fn from_client(client: USaintClient) -> Result<Self, RusaintError>;
}

/// 새로운 [`USaintClient`]를 생성하는 빌더
pub struct USaintClientBuilder {
    session: Option<Arc<USaintSession>>,
}

impl USaintClientBuilder {
    /// 새로운 빌더를 만듭니다.
    pub fn new() -> USaintClientBuilder {
        USaintClientBuilder { session: None }
    }

    /// 빌더에 [`USaintSession`]을 추가합니다.
    pub fn session(mut self, session: Arc<USaintSession>) -> USaintClientBuilder {
        self.session = Some(session);
        self
    }

    /// 애플리케이션 이름과 함께 [`USaintClient`]을 생성합니다.
    pub async fn build(self, name: &str) -> Result<USaintClient, WebDynproError> {
        let mut builder = WebDynproClientBuilder::new(SSU_WEBDYNPRO_BASE_URL, name);
        if let Some(session) = self.session {
            let client = reqwest::Client::builder()
                .cookie_provider(session)
                .user_agent(DEFAULT_USER_AGENT)
                .build()
                .unwrap();
            builder = builder.client(client);
        }
        let base_app = builder.build().await?;
        USaintClient::new(base_app).await
    }

    /// 특정 [`USaintApplication`]을 만듭니다.
    pub async fn build_into<T: USaintApplication>(self) -> Result<T, RusaintError> {
        let name = T::APP_NAME;
        let client = self.build(name).await?;
        T::from_client(client)
    }
}
/// 학생 성적 조회: [`CourseGrades`](course_grades::CourseGrades)
pub mod course_grades;

/// 강의시간표: [`CourseSchedule`](course_schedule::CourseSchedule)
pub mod course_schedule;

/// 졸업사정표: [`GraduationRequirements`]
pub mod graduation_requirements;
mod student_information;
