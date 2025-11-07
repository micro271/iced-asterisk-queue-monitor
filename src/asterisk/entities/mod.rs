pub mod caller;
pub mod member;

/// This struct represent one queue and its state
/// Event: QueueParams
/// 
/// Queue: queue name
/// calls: active calls
/// 
#[derive(Debug)]
pub struct Params {
    queue: String,
    calls: u32, // llamadas en cola
    hold_time: u64, //promedio de tiempo en espera
    talk_time: u64, // promedio de tiempo en conversacion
    completed: u32, // llamadas atendidas
    abandoned: u32, // llamadas abandonadasq
}


// Caller in queue
#[derive(Debug)]
pub struct Entry {
    queue: String,
    caller_number: String,
    wait: u64,
    priprity: String,
    unique_id: u32
}


#[derive(Debug)]
pub enum Membership {
    Dynamic,
    Static,
}