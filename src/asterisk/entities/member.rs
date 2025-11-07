use crate::asterisk::entities::Membership;

///
/// Queue user information
/// This information is received when we are connected to the queue event
/// 
/// Queue: queue name
/// 
#[derive(Debug)]
pub struct MemberStatus {
    queue: String,
    location: String,
    calls_taken: u16,
    membership: Membership,
    last_call: u64,
    in_call: bool,
    paused: bool,
    status: Status,
    pause_reason: String,
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
    membership: Membership,
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