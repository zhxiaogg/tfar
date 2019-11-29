use super::StateMachine;
use crate::state_machine::{
    events::{StateEvent, StateEvent::*},
    states::{InternalState, LeaderVolatileState, LogEntry, LogEntryIndex, PersistentState, ServerId, ServerVolatileState, TermId, VoteResult},
};
use async_trait::async_trait;

pub struct Leader {
    persistent:      PersistentState,
    volatile:        ServerVolatileState,
    leader_volatile: LeaderVolatileState,
    internal:        InternalState,
}

#[async_trait]
impl StateMachine for Leader {
    async fn on_events(self, event: StateEvent) -> Box<dyn StateMachine> {
        match event {
            Timeout(_) => panic!("Leader received Timeout event!"),
            _ => panic!("unrecognized evnet!"),
        }
    }
}
