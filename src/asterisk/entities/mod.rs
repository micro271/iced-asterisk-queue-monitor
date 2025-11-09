use crate::asterisk::event::ParserEvent;

pub mod caller;
pub mod member;
pub mod agent;

/// This struct represent one queue and its state
/// Event: QueueParams
///
/// Queue: queue name
/// calls: active calls
///
#[derive(Debug)]
pub struct Params {
    queue: String,
    calls: u32,     // llamadas en cola
    hold_time: u64, //promedio de tiempo en espera
    talk_time: u64, // promedio de tiempo en conversacion
    completed: u32, // llamadas atendidas
    abandoned: u32, // llamadas abandonadasq
}

impl ParserEvent for Params {
    fn parse_from_map(data: std::collections::HashMap<&str, &str>) -> Self
    where
        Self: Sized,
    {
        Self {
            queue: data.get("Queue").unwrap().to_string(),
            calls: data.get("Calls").unwrap().parse().unwrap(),
            hold_time: data.get("Holdtime").unwrap().parse().unwrap(),
            talk_time: data.get("TalkTime").unwrap().parse().unwrap(),
            completed: data.get("Completed").unwrap().parse().unwrap(),
            abandoned: data.get("Abandoned").unwrap().parse().unwrap(),
        }
    }
}

// Caller in queue
#[derive(Debug)]
pub struct Entry {
    queue: String,
    caller_number: String,
    wait: u64,
    priprity: String,
    unique_id: u32,
}
