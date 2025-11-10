use macros::ParserEvent;

/// Queue: Queue name
/// Position: Queue position
/// CallerIDNum: Caller number
/// CallerIDName: Caller name
/// WaitTime: Time spent waiting
/// Uniqueid
#[derive(Debug, ParserEvent)]
pub struct Caller {
    #[parser(key = "Queue")]
    pub queue: String,

    #[parser(key = "Position", use_parse)]
    pub position: u16,

    #[parser(key = "CallerIDNum")]
    pub caller_id_num: String,

    #[parser(key = "CallerIDName")]
    pub caller_id_name: String,

    #[parser(key = "Uniqueid")]
    pub callet_unique_id: String,

    #[parser(key = "HoldTime")]
    pub hold_time: String,

    #[skip_with_defaut]
    pub r#type: TypeCallerEvent,
}

impl Caller {
    pub fn r#type(mut self, r#type: TypeCallerEvent) -> Self {
        self.r#type = r#type;
        self
    }
}

#[derive(Debug, Default)]
pub enum TypeCallerEvent {
    /// the time field represent the time spend wating
    Join,

    /// Represent the HoldTime event of an caller
    /// it's the number of time that the caller left the queue
    Leave,

    /// Indicates how long the caller has been waiting
    Abandon,

    #[default]
    Unknown,
}

impl std::fmt::Display for TypeCallerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeCallerEvent::Join => write!(f, "CallerJoin"),
            TypeCallerEvent::Leave => write!(f, "CallerLeave"),
            TypeCallerEvent::Abandon => write!(f, "CallerAbandon"),
            TypeCallerEvent::Unknown => write!(f, "Unknown"),
        }
    }
}
