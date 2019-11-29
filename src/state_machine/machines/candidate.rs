use super::{follower::Follower, StateMachine};
use crate::state_machine::{
    events::{StateEvent, StateEvent::*},
    states::{InternalState, LogEntry, LogEntryIndex, PersistentState, ServerId, ServerVolatileState, TermId, VoteResult},
};
use async_trait::async_trait;

pub struct Candidate {
    persistent: PersistentState,
    volatile:   ServerVolatileState,
    internal:   InternalState,
}

#[async_trait]
impl StateMachine for Candidate {
    async fn on_events(self, event: StateEvent) -> Box<dyn StateMachine> {
        match event {
            Timeout(_) => Box::new(self.become_candidate().await),
            VoteResponse { term, vote_granted, server_id } if term <= self.term() => Box::new(self.with_vote(vote_granted, server_id)),
            VoteResponse { term, vote_granted, server_id } if term > self.term() => Box::new(self.become_follower(term).await),
            _ => panic!("unrecognized event."),
        }
    }
}

impl Candidate {
    pub fn new(persistent: PersistentState, volatile: ServerVolatileState, internal: InternalState) -> Candidate {
        Candidate { persistent, volatile, internal }
    }

    pub fn term(&self) -> TermId {
        self.persistent.term()
    }

    /// create a new candidate by updating current one with a vote response
    fn with_vote(self, granted: bool, server_id: ServerId) -> Candidate {
        let Candidate { persistent, volatile, internal } = self;
        Candidate {
            persistent: persistent,
            volatile:   volatile,
            internal:   internal.with_vote(server_id, granted),
        }
    }

    /// turn candidate into follower after failed voting
    async fn become_follower(self, term: TermId) -> Follower {
        // TODO: write storage
        let Candidate { persistent, volatile, internal } = self;
        Follower::new(persistent.with_new_term(term), volatile, internal.clear_voting())
    }

    /// turn current candidate into a new candidate with term increased
    async fn become_candidate(self) -> Candidate {
        let Candidate { persistent, volatile, internal } = self;
        // TODO: write storage
        Candidate {
            persistent: persistent.incr_term(),
            volatile:   volatile,
            internal:   internal.clear_voting(),
        }
    }
}
