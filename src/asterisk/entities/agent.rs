

/// Raised when an queue member is notified of a caller in the queue.
#[derive(Debug)]
pub struct AgenteCalled {
    unique_id: String,
    queue: String,
    member_name: String,
    interface: String,
    caller_id_num: String,
    caller_id_name: String,
    ext: String,
    dest_caller_id: String,
    dest_caller_name: String,
    dest_connected_line_num: String,
    dest_connected_line_name: String,
}


/// Raised when a queue member answers and is bridged to a caller in the queue.
#[derive(Debug)]
pub struct AgentConnect {
    queue: String,
    caller_unique_id: String,
    caller_id_num: String,
    caller_id_name: String,
    des_caller_id_num: String,
    des_caller_id_name: String,
    des_connected_line_num: String,
    des_connected_line_name: String,
    interface: String,
    member_name: String,
    ring_time: u64,
    hold_time: u64,
}

// Raised when a queue member has finished servicing a caller in the queue.
#[derive(Debug)]
pub struct AgentCompleted {
    queue: String,
    membe_name: String,
    interface: String,
    hold_time: u64,
    talk_tim: u64,
    caller_unique_id: String,
    caller_id_number: String,
    caller_id_name: String,
    dest_caller_id_num: String,
    dest_caller_id_name: String,
    des_connected_line_num: String,
    des_connected_line_name: String,
    dest_unique_id: String,
}

//Raised when a queue member is notified of a caller in the queue and fails to answer.
#[derive(Debug)]
pub struct AgentRingNoAnswer {
    queue: String,
    name_member: String,
    interface: String,
    ring_time: u64,
    caller_id_num: String,
    caller_id_name: String,
    unique_id: String,
    dest_caller_id_num: String,
    dest_caller_id_name: String,
    dest_connected_line_num: String,
    dest_connected_line_name: String,
    des_unique_id: String,
}

// Raised when a queue member hangs up on a caller in the queue.
#[derive(Debug)]
pub struct AgentDump;

// Raised when an Agent has logged in.
#[derive(Debug)]
pub struct AgentLogin;

// Raised when an Agent has logged off.
#[derive(Debug)]
pub struct AgentLogoff;