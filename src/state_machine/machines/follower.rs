use super::{candidate::Candidate, StateMachine};
use crate::state_machine::{
    events::StateEvent,
    states::{InternalState, LogEntry, LogEntryIndex, PersistentState, ServerId, ServerVolatileState, TermId, VoteResult},
};
use async_trait::async_trait;
use log::{debug, error, info};
use std::time::Duration;

pub struct Follower {
    persistent: PersistentState,
    volatile:   ServerVolatileState,
    internal:   InternalState,
}

impl Follower {
    pub fn new(persistent: PersistentState, volatile: ServerVolatileState, internal: InternalState) -> Follower {
        Follower { persistent, volatile, internal }
    }
}

#[async_trait]
impl StateMachine for Follower {
    async fn on_events(self, event: StateEvent) -> Box<dyn StateMachine> {
        use StateEvent::*;
        match event {
            Timeout(_) => Box::new(self.become_candidate().await),
            VoteRequest { term, candidate, last_log } => {
                if self.persistent.accept_vote(term, candidate, &last_log) {
                    // we granted the vote request
                    Box::new(self.vote_for(term, candidate).await)
                } else if self.persistent.accept_term(term) {
                    // we denied the request, but we found a new term.
                    // TODO: make sure this is supported by the paper
                    Box::new(self.new_term(term).await)
                } else {
                    // we denied the vote request
                    Box::new(self)
                }
            },
            _ => panic!(""),
        }
    }
}

impl Follower {
    async fn become_candidate(self) -> Candidate {
        // TODO: write storage
        let Follower { persistent, volatile, internal } = self;
        Candidate::new(persistent.incr_term(), volatile, internal.clear_voting())
    }

    async fn new_term(self, term: TermId) -> Follower {
        // TODO: write storage
        let Follower { persistent, volatile, internal } = self;
        Follower {
            persistent: persistent.with_new_term(term),
            volatile:   volatile,
            internal:   internal,
        }
    }

    async fn vote_for(self, term: TermId, candidate: ServerId) -> Follower {
        // TODO: write storage
        let Follower { persistent, volatile, internal } = self;
        Follower {
            persistent: persistent.with_new_term(term).with_vote_for(candidate),
            volatile:   volatile,
            internal:   internal,
        }
    }
}
