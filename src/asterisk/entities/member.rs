use std::collections::HashMap;
use crate::asterisk::event::ParserEvent;

///
/// Queue user information
/// This information is received when we are connected to the queue event
///
/// Queue: queue name
///
#[derive(Debug)]
pub struct QueueMember {
    queue: String,
    interface: String,
    member_name: String,
    status: Status,
    log_in_time: String,
    last_call: u64,
    last_pause: u64,
    calls_taken: u16,
    in_call: bool,
    ring_in_use: bool,
    pause_reason: String,
    paused: bool,
}

impl ParserEvent for QueueMember {
    fn parse_from_map(mut map: HashMap<&str, &str>) -> Self
    where
        Self: Sized,
    {
        Self {
            queue: map.remove("Queue").unwrap().to_string(),
            interface: map.remove("Interface").or_else(|| map.remove("StateInterface")).map(ToString::to_string).unwrap_or_default(),
            log_in_time: map.remove("LoginTime").map(ToString::to_string).unwrap_or_default(),
            last_pause: map.remove("LastPause").unwrap().parse().unwrap_or_default(),
            calls_taken: map
                .remove("CallsTaken")
                .and_then(|x| x.parse().ok())
                .unwrap_or_default(),
            member_name: map.remove("MemberName").or_else(||map.remove("Name")).map(ToString::to_string).unwrap_or_default(),
            last_call: map.remove("LastCall").unwrap().parse().unwrap_or_default(),
            in_call: map.remove("InCall").map(|x| x == "1").unwrap_or_default(),
            paused: map.remove("Paused").map(|x| x == "1").unwrap_or_default(),
            ring_in_use: map.remove("Ringinuse").map(|x| x == "1").unwrap_or_default(),
            status: map.remove("Status").unwrap().try_into().unwrap_or_default(),
            pause_reason: map.remove("PausedReason").map(ToString::to_string).unwrap_or_default(),
        }
    }
}

// Member status
#[derive(Debug, Default)]
enum Status {
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

impl TryFrom<&str> for Status {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        
        match value {
            "0" => Ok(Self::Unknown),
            "1" => Ok(Self::NotInUse),
            "2" => Ok(Self::InUse),
            "3" => Ok(Self::Busy),
            "4" => Ok(Self::Invalid),
            "5" => Ok(Self::Unavailable),
            "6" => Ok(Self::Ringing),
            "7" => Ok(Self::RingingAndInUse),
            "8" => Ok(Self::OnHold),
            _ => Err(())
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
#[derive(Debug)]
pub struct MemberRingnoanswer {
    queue: String,
    interface: String,
    member_name: String,
    position: u16,
    caller_id: String,
    caller_name: String,
}

/// Event: QueueMemberBusy
#[derive(Debug)]
pub struct MemberBusy {
    queue: String,
    interface: String,
    member_name: String,
    position: u16,
    caller_id: String,
    caller_name: String,
}
