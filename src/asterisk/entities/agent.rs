use macros::ParserEvent;

/// Raised when an queue member is notified of a caller in the queue.
#[derive(Debug, ParserEvent)]
pub struct AgenteCalled {
    #[parser(key = "Queue")]
    pub queue: String,

    #[parser(key = "MemberName")]
    pub member_name: String,

    #[parser(key = "Interface")]
    pub interface: String,

    #[parser(key = "Uniqueid")]
    pub caller_unique_id: String,

    #[parser(key = "CallerIDNum")]
    pub caller_id_num: String,

    #[parser(key = "CallerIDName")]
    pub caller_id_name: String,

    #[parser(key = "DestUniqueid")]
    pub dest_unique_id: String,

    #[parser(key = "DestCallerIDNum")]
    pub dest_caller_id_num: String,

    #[parser(key = "DestCallerIDName")]
    pub dest_caller_id_name: String,

    #[parser(key = "DestConnectedLineNum")]
    pub dest_connected_line_num: String,

    #[parser(key = "DestConnectedLineName")]
    pub dest_connected_line_name: String,
}

/// Raised when a queue member answers and is bridged to a caller in the queue.
#[derive(Debug, ParserEvent)]
pub struct AgentConnect {
    #[parser(key = "Queue")]
    pub queue: String,

    #[parser(key = "Uniqueid")]
    pub caller_unique_id: String,

    #[parser(key = "CallerIDNum")]
    pub caller_id_num: String,

    #[parser(key = "CallerIDName")]
    pub caller_id_name: String,

    #[parser(key = "DestUniqueid")]
    pub dest_unique_id: String,

    #[parser(key = "DestCallerIDNum")]
    pub dest_caller_id_num: String,

    #[parser(key = "DestCallerIDName")]
    pub dest_caller_id_name: String,

    #[parser(key = "DestConnectedLineNum")]
    pub dest_connected_line_num: String,

    #[parser(key = "DestConnectedLineName")]
    pub dest_connected_line_name: String,

    #[parser(key = "Interface")]
    pub interface: String,

    #[parser(key = "MemberName")]
    pub member_name: String,

    #[parser(key = "RingTime", use_parse)]
    pub ring_time: u64,

    #[parser(key = "HoldTime", use_parse)]
    pub hold_time: u64,
}

// Raised when a queue member has finished servicing a caller in the queue.
#[derive(Debug, ParserEvent)]
pub struct AgentComplete {
    #[parser(key = "Queue")]
    pub queue: String,

    #[parser(key = "MemberName")]
    pub member_name: String,

    #[parser(key = "Interface")]
    pub interface: String,

    #[parser(key = "HoldTime", use_parse)]
    pub hold_time: u64,

    #[parser(key = "TalkTime", use_parse)]
    pub talk_time: u64,

    #[parser(key = "Uniqueid")]
    pub caller_unique_id: String,

    #[parser(key = "CallerIDNum")]
    pub caller_id_num: String,

    #[parser(key = "CallerIDName")]
    pub caller_id_name: String,

    #[parser(key = "DestCallerIDNum")]
    pub dest_caller_id_num: String,

    #[parser(key = "DestCallerIDName")]
    pub dest_caller_id_name: String,

    #[parser(key = "DestConnectedLineNum")]
    pub dest_connected_line_num: String,

    #[parser(key = "DestConnectedLineName")]
    pub dest_connected_line_name: String,

    #[parser(key = "DestUniqueid")]
    pub dest_unique_id: String,

    #[parser(key = "Reason")]
    pub reason: String,
}

//Raised when a queue member is notified of a caller in the queue and fails to answer.
#[derive(Debug, ParserEvent)]
pub struct AgentRingNoAnswer {
    #[parser(key = "Queue")]
    pub queue: String,

    #[parser(key = "MemberName")]
    pub member_name: String,

    #[parser(key = "Interface")]
    pub interface: String,

    #[parser(key = "RingTime", use_parse)]
    pub ring_time: u64,

    #[parser(key = "Uniqueid")]
    pub caller_unique_id: String,

    #[parser(key = "CallerIDNum")]
    pub caller_id_num: String,

    #[parser(key = "CallerIDName")]
    pub caller_id_name: String,

    #[parser(key = "DestCallerIDNum")]
    pub dest_caller_id_num: String,

    #[parser(key = "DestCallerIDName")]
    pub dest_caller_id_name: String,

    #[parser(key = "DestConnectedLineNum")]
    pub dest_connected_line_num: String,

    #[parser(key = "DestConnectedLineName")]
    pub dest_connected_line_name: String,

    #[parser(key = "DestUniqueid")]
    pub dest_unique_id: String,
}

// Raised when a queue member hangs up on a caller in the queue.
#[derive(Debug, ParserEvent)]
pub struct AgentDump {
    #[parser(key = "Queue")]
    pub queue: String,

    #[parser(key = "MemberName")]
    pub member_name: String,

    #[parser(key = "Interface")]
    pub interface: String,

    #[parser(key = "Uniqueid")]
    pub caller_unique_id: String,

    #[parser(key = "CallerIDNum")]
    pub caller_id_num: String,

    #[parser(key = "CallerIDName")]
    pub caller_id_name: String,

    #[parser(key = "DestUniqueid")]
    pub dest_caller_id_num: String,

    #[parser(key = "DestCallerIDNum")]
    pub dest_caller_id_name: String,

    #[parser(key = "DestCallerIDName")]
    pub dest_unique_id: String,
}

// Raised when an Agent has logged in.
#[derive(Debug)]
pub struct AgentLogin {
    // ...
}

// Raised when an Agent has logged off.
#[derive(Debug)]
pub struct AgentLogoff;
