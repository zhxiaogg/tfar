use super::{follower::Follower, StateMachine};
use crate::state_machine::{
    events::{StateEvent, StateEvent::*},
    states::{InternalState, LeaderVolatileState, LogEntry, LogEntryId, LogEntryIndex, PersistentState, ServerId, ServerVolatileState, TermId, VoteResult},
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
            VoteRequest { term, candidate, last_log } => {
                if self.accept_vote(term, candidate, &last_log) {
                    // we granted the vote request
                    Box::new(self.become_follower_and_vote_for(term, candidate).await)
                } else if term > self.term() {
                    // we denied the request, but we found a new term.
                    Box::new(self.become_follower(term).await)
                } else {
                    // we denied the vote request
                    Box::new(self)
                }
            },
            VoteResponse {..} => panic!("Leader received vote response event!"),
            AppendEntriesRequest { term, leader, prev_log, entries, commit_idx } => {
                let is_new_leader = term > self.term();
                let accept_logs = self.accept_logs(&prev_log);

                if is_new_leader && accept_logs {
                    Box::new(self.become_follower_on_new_leader_with_logs(term, leader, prev_log, entries, commit_idx).await)
                } else if is_new_leader {
                    Box::new(self.become_follower_on_new_leader(term, leader, commit_idx).await)
                } else {
                    Box::new(self)
                }
            },
            AppendEntriesResponse { term,  success, server} => {
                if term > self.term()  {
                    // found a new leader
                    Box::new(self.become_follower(term ).await)
                } else {
                    // update appending entries status
                    panic!("not implemented yet!")
                }
            }, 
            _ => panic!("unrecognized evnet!"),
        }
    }
}

impl Leader {
    fn term(&self) -> TermId {
        self.persistent.term()
    }

    fn accept_vote(&self, term: TermId, candidate: ServerId, last_log: &LogEntryId) -> bool {
        let candidate_match = self.persistent.accept_candidate(candidate);
        let newer_log = self.persistent.accept_log(last_log);
        term > self.term() && candidate_match && newer_log
    }

    fn accept_logs(&self, prev_log:&LogEntryId) -> bool {
        self.persistent.accept_log(prev_log)
    }

    async fn become_follower_and_vote_for(self, term: TermId, candidate: ServerId) -> Follower {
        // TODO: write storage
        let Leader { persistent, volatile, internal, .. } = self;
        Follower::new(persistent.with_vote(term, candidate), volatile, internal)
    }

    async fn become_follower(self, term: TermId) -> Follower {
        // TODO: write storage
        let Leader { persistent, volatile, internal, .. } = self;
        Follower::new(persistent.with_new_term(term), volatile, internal)
    }

    async fn become_follower_on_new_leader_with_logs(self, term: TermId, leader: ServerId, prev_log: LogEntryId, entries: Vec<LogEntry>, leader_commit: LogEntryIndex) -> Follower {
        // TODO: write storage
        let Leader { persistent, volatile, internal,.. } = self;
        let persistent = persistent.with_new_term(term).with_log_entries(prev_log, entries);
        let volatile = volatile.with_commit_index(leader_commit);
        let internal = internal.with_leader(leader);
        Follower::new(persistent, volatile, internal)
    }

    async fn become_follower_on_new_leader(self, term: TermId, leader: ServerId, leader_commit: LogEntryIndex) -> Follower {
        // TODO: write storage
        let Leader { persistent, volatile, internal, .. } = self;
        let persistent = persistent.with_new_term(term);
        let volatile = volatile.with_commit_index(leader_commit);
        let internal = internal.with_leader(leader);
        Follower::new(persistent, volatile, internal)
    }
}
