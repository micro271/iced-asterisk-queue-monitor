use macros::ParserEvent;

pub mod agent;
pub mod caller;
pub mod member;

/// This struct represent one queue and its state
/// Event: QueueParams
///
/// Queue: queue name
/// calls: active calls
#[derive(Debug, ParserEvent)]
pub struct Params {
    #[parser(key = "Queue")]
    pub queue: String,

    #[parser(use_parse, key = "Calls")]
    pub calls: u32, // llamadas en cola

    #[parser(use_parse, key = "Holdtime")]
    pub hold_time: u64, //promedio de tiempo en espera

    #[parser(use_parse, key = "TalkTime")]
    pub talk_time: u64, // promedio de tiempo en conversacion

    #[parser(use_parse, key = "Completed")]
    pub completed: u32, // llamadas atendidas

    #[parser(use_parse, key = "Abandoned")]
    pub abandoned: u32, // llamadas abandonadasq
}

// Caller in queue
#[derive(Debug, ParserEvent)]
pub struct Entry {
    #[parser(key = "Queue")]
    pub queue: String,

    #[parser(key = "CallerIDNum")]
    pub caller_id_number: String,

    #[parser(key = "CallerIDName")]
    pub caller_id_name: String,

    #[parser(key = "ConnectedLineNum")]
    pub connected_line_num: String,

    #[parser(key = "ConnectedLineName")]
    pub connected_line_name: String,

    #[parser(key = "Wait", use_parse)]
    pub wait: u64,

    #[parser(key = "Uniqueid")]
    pub unique_id: String,
}

#[derive(Debug, ParserEvent)]
pub struct StatusComplete {
    #[parser(key = "ListItems", use_parse)]
    pub len: i32,
}

#[derive(Debug, ParserEvent)]
pub struct ResponseAmi {
    #[parser(key = "Response")]
    pub response: ResponseAmiResult,

    #[parser(key = "Message", key = "Events")]
    pub message: String,

    #[skip_with_defaut]
    pub r#type: ResponseAmyType,
}

impl ResponseAmi {
    pub fn r#type(mut self, r#type: ResponseAmyType) -> Self {
        self.r#type = r#type;
        self
    }
}

#[derive(Debug, Default)]
pub enum ResponseAmyType {
    Login,
    Action,
    #[default]
    Unknown
}

impl ResponseAmi {
    fn is_ok(&self) -> bool {
        self.response == ResponseAmiResult::Success
    }
}

#[derive(Debug, PartialEq, Default)]
pub enum ResponseAmiResult {
    Error,
    Success,

    #[default]
    Unknown,
}

impl From<&str> for ResponseAmiResult {
    fn from(value: &str) -> Self {
        match value {
            "Success" => Self::Success,
            "Error" => Self::Error,
            _ => Self::Unknown,
        }
    }
}
