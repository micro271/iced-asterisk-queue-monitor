/// Queue: Queue name
/// Position: Queue position
/// CallerIDNum: Caller number
/// CallerIDName: Caller name
/// WaitTime: Time spent waiting
/// Uniqueid
///
#[derive(Debug)]
pub struct Caller {
    queue: String,
    position: u16,
    caller_id: String,
    caller_name: String,
    time: String,
    r#type: TypeCallerEvent,
}

#[derive(Debug)]
enum TypeCallerEvent {
    /// the time field represent the time spend wating
    Join,

    /// Represent the HoldTime event of an caller
    /// it's the number of time that the caller left the queue
    Leave,

    /// Indicates how long the caller has been waiting
    Abandon,

    Reconnect,
}

impl std::fmt::Display for TypeCallerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeCallerEvent::Join => write!(f, "CallerJoin"),
            TypeCallerEvent::Leave => write!(f, "CallerLeave"),
            TypeCallerEvent::Abandon => write!(f, "CallerAbandon"),
            TypeCallerEvent::Reconnect => write!(f, "CallerReconnect"),
        }
    }
}
