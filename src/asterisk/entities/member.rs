use std::collections::HashMap;

use iced::keyboard::key;

use crate::asterisk::event::ParserEvent;

///
/// Queue user information
/// This information is received when we are connected to the queue event
///
/// Queue: queue name
///
#[derive(Debug)]
pub struct MemberStatus {
    queue: String,
    log_in_time: String,
    member_name: String,
    calls_taken: u16,
    last_call: u64,
    in_call: bool,
    interface: String,
    last_pause: u64,
    paused: bool,
    status: Status,
    pause_reason: String,
}

impl ParserEvent for MemberStatus {
    fn parse_from_map(mut map: HashMap<&str, &str>) -> Self
    where
        Self: Sized,
    {
        Self {
            queue: map.remove("Queue").unwrap().to_string(),
            interface: map.remove("Interface").unwrap_or(map.remove("StateInterface").unwrap()).to_string(),
            log_in_time: map.remove("LoginTime").unwrap().to_string(),
            last_pause: map.remove("LastPause").unwrap().parse().unwrap_or_default(),
            calls_taken: map
                .remove("CallsTaken")
                .and_then(|x| x.parse().ok())
                .unwrap(),
            member_name: map.remove("MemberName").unwrap_or(map.remove("Name").unwrap_or_default()).to_string(),
            last_call: map.remove("LastCall").unwrap().parse().unwrap(),
            in_call: map.remove("InCall").map(|x| x == "1").unwrap_or_default(),
            paused: map.remove("Paused").map(|x| x == "1").unwrap_or_default(),
            status: map.remove("Status").unwrap().try_into().unwrap(),
            pause_reason: map.remove("PausedReason").unwrap().to_string(),
        }
    }
}

// Member status
#[derive(Debug)]
enum Status {
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
            "0" => Ok(Self::NotInUse),
            "1" => Ok(Self::OnHold),
            "2" => Ok(Self::InUse),
            "3" => Ok(Self::Busy),
            "4" => Ok(Self::Unavailable),
            "5" => Ok(Self::RingingAndInUse),
            "6" => Ok(Self::Ringing),
            "7" => Ok(Self::Unknown),
            "8" => Ok(Self::Invalid),
            _ => Err(())
        }
    }
}

///
/// The agent is paused
///
/// Queue: name of the queue
/// Interface: Agnet/endpoint
/// Paused:
///     0: Not paused
///     1: Paused
/// Reason: ...
#[derive(Debug)]
pub struct MemberPaused {
    queue: String,
    interface: String,
    paused: bool,
    reason: String,
}

/// New agent in the queue
/// Queue: queue name
/// Interface: agent or endpoint
/// Membership:
///     - static
///     - dynamic
/// Penalty: priority in the queue
/// Paused:
///     0: not paused
///     1: pased
#[derive(Debug)]
pub struct MemberAdded {
    queue: String,
    interface: String,
    penalty: u16,
    paused: bool,
}

/// It's asigned an caller to member
///
/// Queue: queue name
/// Interface: agent or endpoint
/// CallerIDNum: caller number
/// CallerIDName: caller name
/// Uniqueid:
/// Position: queue position
#[derive(Debug)]
pub struct MemberCaller {
    queue: String,
    interface: String,
    caller_id: String,
    caller_name: String,
    unique_id: String,
    position: u16,
}

///
/// An member answer a call
/// Queue: queue name
/// Interface: agent or endpoint
/// Uniqueid:
/// HoldTime: waited time to answer the caller
/// talk time: conversation time
#[derive(Debug)]
pub struct MemberConnect {
    queue: String,
    interface: String,
    caller_id: String,
    caller_name: String,
    unique_id: String,
    hole_time: u64,
    talk_time: u64,
}

///
/// Queue: Queue name
/// Interface: agent or endpoint
/// Reason: Optional field, indicate the reason
#[derive(Debug)]
pub struct MemberRemoved {
    queue: String,
    interface: String,
    reason: Option<String>,
}

///
/// Finished caller
/// Queue: queue name
/// Interface: agent or endpoint
/// CallerIDNum: caller number
/// CallerIDName: caller name
/// Uniqueid:
/// HoldTime: the time waiting
/// TalkTime: Conversation time
#[derive(Debug)]
pub struct MemberComplete {
    queue: String,
    interface: String,
    caller_id: String,
    caller_name: String,
    unique_id: String,
    hole_time: u64,
    talk_time: u64,
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
