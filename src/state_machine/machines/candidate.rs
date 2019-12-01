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
            VoteResponse { term, vote_granted, server_id } => {
                if term <= self.term() {
                    Box::new(self.with_vote(vote_granted, server_id))
                } else {
                    Box::new(self.become_follower(term).await)
                }
            },
            VoteRequest {
                term,
                candidate,
                last_log_index,
                last_log_term,
            } => {
                if term > self.term() && self.persistent.can_vote(term, candidate, last_log_index, last_log_term) {
                    // we granted the vote request
                    Box::new(self.become_follower_and_vote_for(term, candidate).await)
                } else if term > self.term() {
                    // we denied the request, but we found a new term.
                    Box::new(self.become_follower(term).await)
                } else if self.persistent.can_vote(term, candidate, last_log_index, last_log_term) {
                    // we granted the vote request
                    Box::new(self.vote_for(term, candidate).await)
                } else {
                    // we denied the vote request
                    Box::new(self)
                }
            },
            AppendEntriesRequest {
                term,
                leader_id,
                prev_log_index,
                prev_log_term,
                entries,
                leader_commit,
            } => {
                if term < self.term() {
                    // deny the request
                    Box::new(self)
                } else if self.accept_leader_state(prev_log_index, prev_log_term) {
                    // found a new leader
                    Box::new(self.become_follower_on_new_leader_with_entries(term, leader_id, prev_log_index, prev_log_term, entries, leader_commit))
                } else {
                    // found a new leader, but the state didn't match
                    Box::new(self.become_follower_on_new_leader(term, leader_id, leader_commit))
                }
            },
            AppendEntriesResponse { .. } => panic!("unrecognized event."),
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

    async fn become_follower_and_vote_for(self, term: TermId, candidate_id: ServerId) -> Follower {
        // TODO: write storage
        let Candidate { persistent, volatile, internal } = self;
        Follower::new(persistent.with_vote(term, candidate_id), volatile, internal.clear_voting())
    }

    async fn vote_for(self, term: TermId, candidate_id: ServerId) -> Candidate {
        // TODO: write storage
        let Candidate { persistent, volatile, internal } = self;
        Candidate {
            persistent: persistent.with_vote(term, candidate_id),
            volatile:   volatile,
            internal:   internal,
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

    fn accept_leader_state(&self, prev_log_index: LogEntryIndex, prev_log_term: TermId) -> bool {
        self.persistent.accept_log(prev_log_index, prev_log_term)
    }

    fn become_follower_on_new_leader_with_entries(self, term: TermId, leader_id: ServerId, prev_log_index: LogEntryIndex, prev_log_term: TermId, entries: Vec<LogEntry>, leader_commit: LogEntryIndex) -> Follower {
        // TODO: write storage
        let Candidate { persistent, volatile, internal } = self;
        let persistent = persistent.with_new_term(term).with_log_entries(entries);
        let volatile = volatile.with_commit_index(leader_commit);
        let internal = internal.clear_voting();
        Follower::new(persistent, volatile, internal)
    }

    fn become_follower_on_new_leader(self, term: TermId, leader: ServerId, leader_commit: LogEntryIndex) -> Follower {
        // TODO: write storage
        let Candidate { persistent, volatile, internal } = self;
        let persistent = persistent.with_new_term(term);
        let volatile = volatile.with_commit_index(leader_commit);
        let internal = internal.clear_voting();
        Follower::new(persistent, volatile, internal)
    }
}
