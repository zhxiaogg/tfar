use super::{candidate::Candidate, StateMachine};
use crate::state_machine::{
    events::StateEvent,
    states::{InternalState, LogEntry, LogEntryId, LogEntryIndex, PersistentState, ServerId, ServerVolatileState, TermId, VoteResult},
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
                if self.accept_vote(term, candidate, &last_log) {
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
            VoteResponse { .. } => panic!("follower receive vote response!"),
            AppendEntriesRequest { term, leader, prev_log, entries, commit_idx } => {
                let is_new_leader = self.accept_new_leader(term);
                let accept_logs = self.accept_logs(term, leader, &prev_log);

                if is_new_leader && accept_logs {
                    Box::new(self.new_leader_with_logs(term, leader, prev_log, entries, commit_idx).await)
                } else if accept_logs {
                    Box::new(self.append_logs(prev_log, entries, commit_idx).await)
                } else if is_new_leader {
                    Box::new(self.new_leader(term, leader, commit_idx).await)
                } else {
                    Box::new(self)
                }
            },
            AppendEntriesResponse { .. } => panic!("follower received append entries response!"),
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

    fn accept_new_leader(&self, term: TermId) -> bool {
        term > self.persistent.term()
    }

    fn accept_vote(&self, term: TermId, candidate: ServerId, last_log: &LogEntryId) -> bool {
        let candidate_match = self.persistent.accept_candidate(candidate);
        let newer_log = self.persistent.accept_log(last_log);
        term >= self.persistent.term() && candidate_match && newer_log
    }

    fn accept_logs(&self, term: TermId, server: ServerId, prev_log: &LogEntryId) -> bool {
        let accept_log = self.persistent.accept_log(prev_log);
        let accept_server = self.persistent.term() < term || self.persistent.term() == term && self.internal.has_leader(server);
        accept_log && accept_server
    }

    async fn new_leader_with_logs(self, term: TermId, leader: ServerId, prev_log: LogEntryId, entries: Vec<LogEntry>, commit_idx: LogEntryIndex) -> Follower {
        // TODO: write storage
        let Follower { persistent, volatile, internal } = self;
        let persistent = persistent.with_new_term(term).with_log_entries(prev_log, entries);
        let volatile = volatile.with_commit_index(commit_idx);
        let internal = internal.with_leader(leader);
        Follower { persistent, volatile, internal }
    }

    async fn append_logs(self, prev_log: LogEntryId, entries: Vec<LogEntry>, commit_idx: LogEntryIndex) -> Follower {
        // TODO: write storage
        let Follower { persistent, volatile, internal } = self;
        let persistent = persistent.with_log_entries(prev_log, entries);
        let volatile = volatile.with_commit_index(commit_idx);
        Follower { persistent, volatile, internal }
    }

    async fn new_leader(self, term: TermId, leader: ServerId, commit_idx: LogEntryIndex) -> Follower {
        // TODO: write storage
        let Follower { persistent, volatile, internal } = self;
        let persistent = persistent.with_new_term(term);
        let volatile = volatile.with_commit_index(commit_idx);
        let internal = internal.with_leader(leader);
        Follower { persistent, volatile, internal }
    }
}
