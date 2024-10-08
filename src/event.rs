use crate::{Action, TelnetError, TelnetOption};
use std::collections::VecDeque;

/// Events generated by `Telnet`.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Event {
    /// Data received (excluding telnet commands)
    Data(Box<[u8]>),
    /// An unrecognized telnet command received
    UnknownIAC(u8),
    /// A telnet negotiation received
    Negotiation(Action, TelnetOption),
    /// A telnet subnegotiation data received
    Subnegotiation(TelnetOption, Box<[u8]>),
    /// Read time out
    TimedOut,
    /// No data to read
    NoData,
    /// Error encountered during processing read buffer
    Error(TelnetError),
}

#[repr(transparent)]
pub struct TelnetEventQueue(VecDeque<Event>);

impl TelnetEventQueue {
    pub fn new() -> TelnetEventQueue {
        TelnetEventQueue(VecDeque::new())
    }

    pub fn push_event(&mut self, event: Event) {
        self.0.push_back(event);
    }

    pub fn take_event(&mut self) -> Option<Event> {
        self.0.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
