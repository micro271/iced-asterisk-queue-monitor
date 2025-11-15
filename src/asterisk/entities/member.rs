use macros::ParserEvent;

///
/// Queue user information
/// This information is received when we are connected to the queue event
///
/// Queue: queue name
///
#[derive(Debug, ParserEvent)]
pub struct Member {
    #[parser(key = "Queue")]
    pub queue: String,

    #[parser(key = "Interface", key = "StateInterface")]
    pub interface: String,

    #[parser(key = "MemberName", key ="Name")]
    pub member_name: String,

    #[parser(key = "Status")]
    pub status: Status,

    #[parser(key = "LoginTime")]
    pub log_in_time: String,

    #[parser(key = "LastCall", use_parse)]
    pub last_call: u64,

    #[parser(key = "LastPause", use_parse)]
    pub last_pause: u64,

    #[parser(key = "CallsTaken", use_parse)]
    pub calls_taken: u16,

    #[parser(key = "InCall", use_parse)]
    pub in_call: bool,

    #[parser(key = "Ringinuse", use_parse)]
    pub ring_in_use: bool,

    #[parser(key = "PausedReason")]
    pub pause_reason: String,

    #[parser(key = "Paused", use_parse)]
    pub paused: bool,
}

// Member status
#[derive(Debug, Default)]
pub enum Status {
    #[default]
    Unknown,
    NotInUse,
    InUse,
    Busy,
    Invalid,
    Unavailable,
    Ringing,
    RingingAndInUse,
    OnHold,
}

impl From<&str> for Status {
    fn from(value: &str) -> Self {
        match value {
            "1" => Self::NotInUse,
            "2" => Self::InUse,
            "3" => Self::Busy,
            "4" => Self::Invalid,
            "5" => Self::Unavailable,
            "6" => Self::Ringing,
            "7" => Self::RingingAndInUse,
            "8" => Self::OnHold,
            _ => Self::Unknown,
        }
    }
}

/// event: QueueMemberEingNoAnswer
///
/// Queue: queue name
/// Interface: agent or endpoint
/// MemberName: member name
/// Position:
/// CallerIDNum:
/// CallerIDName:
/// Uniqueid:
#[derive(Debug, ParserEvent)]
pub struct MemberRingninuse {
    #[parser(key = "Queue")]
    pub queue: String,

    #[parser(key = "Interface")]
    pub interface: String,

    #[parser(key = "MemberName")]
    pub member_name: String,

    #[parser(key = "Position", use_parse)]
    pub position: u16,

    #[parser(key = "CallerId")]
    pub caller_id: String,

    #[parser(key = "CallerName")]
    pub caller_name: String,
}
