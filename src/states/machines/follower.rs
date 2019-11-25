use super::StateMachine;
use crate::states::{
    events::StateEvent,
    internal::{InternalState, ServerId, VoteResult},
    persistent::{LogEntry, PersistentState, TermId},
    volatile::{LeaderVolatileState, LogEntryIndex, ServerVolatileState},
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
    pub fn new() -> Follower {
        Follower {
            persistent: PersistentState::new(),
            volatile:   ServerVolatileState::new(),
            internal:   InternalState::new(),
        }
    }
}

#[async_trait]
impl StateMachine for Follower {
    async fn on_events(self, event: StateEvent) -> Box<dyn StateMachine> {
        use StateEvent::*;
        match event {
            Timeout(_) => Box::new(self.become_candidate().await),
            VoteRequest {
                term,
                candidate_id,
                last_log_index,
                last_log_term,
            } => {
                if self.persistent.can_vote(term, candidate_id, last_log_index, last_log_term) {
                    // we granted the vote request
                    Box::new(self.vote(term, candidate_id).await)
                } else if self.persistent.is_new_term(term) {
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
    async fn become_candidate(self) -> Follower {
        // TODO: write storage
        let Follower { persistent, volatile, internal } = self;
        Follower {
            persistent: persistent.incr_term(),
            volatile:   volatile,
            internal:   internal,
        }
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

    async fn vote(self, term: TermId, candidate_id: ServerId) -> Follower {
        // TODO: write storage
        let Follower { persistent, volatile, internal } = self;
        Follower {
            persistent: persistent.with_new_term(term).with_vote_for(candidate_id),
            volatile:   volatile,
            internal:   internal,
        }
    }
}
