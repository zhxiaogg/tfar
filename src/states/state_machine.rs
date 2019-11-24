use super::{
    internal::{InternalState, ServerId, VoteResult},
    persistent::{CandidateId, LogEntry, PersistentState, TermId},
    volatile::{LeaderVolatileState, LogEntryIndex, ServerVolatileState},
    ServerState,
    ServerState::*,
};
use std::time::Duration;

/// StateMachine contains the state transition for Raft servers described
/// by the original paper in `Figure 4: Server states`
struct StateMachine {
    state: ServerState,
}

enum StateEvent {
    Timeout(Duration),
    VoteRequest {
        /// candidate's term
        term: TermId,
        /// candidate requesting vote
        candidate_id: CandidateId,
        /// index of candidate's last log entry
        last_log_index: LogEntryIndex,
        /// term of candidate's last log entry
        last_log_term: TermId,
        /// candidator id, this field is an extention by tfar
        candidator_id: ServerId,
    },
    VoteResponse {
        /// Current Term from requested server, for candidate to update itself
        term: TermId,
        /// true means candidate received vote
        vote_granted: bool,
        /// response server, this field is an extention by tfar
        server_id: ServerId,
    },
    AppendEntriesRequest {
        /// leader's term
        term: TermId,
        /// leader id, for followers to redirect clients
        leader_id: ServerId,
        /// index of log entry immediately precedding new entries
        prev_log_index: LogEntryIndex,
        /// term of log entry immediately precedding new entries
        prev_log_term: TermId,
        /// new entries
        entries: Vec<LogEntry>,
        /// leader's commit index
        leader_commit: LogEntryIndex,
    },
    AppendEntriesResponse {
        /// current Term, for leader to update itself
        term: TermId,
        /// true if follower contained entry matching prevLogIndex and prefLogTerm in request
        success: bool,
    },
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            state: Follower {
                persistent: PersistentState::new(),
                volatile:   ServerVolatileState::new(),
                internal:   InternalState::new(),
            },
        }
    }
    pub fn on_message(self, msg: StateEvent) -> StateMachine {
        use StateEvent::*;
        match msg {
            Timeout(timeout) => self.become_candidate(),
            VoteResponse { term, vote_granted, server_id } => {
                if term > self.state.term() {
                    self.become_follower(term)
                } else {
                    self.vote_returned(vote_granted, server_id)
                }
            },
            _ => panic!("not supported!"),
        }
    }

    fn vote_returned(self, vote_granted: bool, server_id: ServerId) -> StateMachine {
        let state = match self.state {
            Candidate { persistent, volatile, internal } => Candidate {
                persistent,
                volatile,
                internal: internal.with_vote(server_id, vote_granted),
            },
            Follower { .. } => panic!("follower received voting response!"),
            Leader { .. } => panic!("leader received voting response!"),
        };
        StateMachine { state }
    }

    fn become_candidate(self) -> StateMachine {
        let state = match self.state {
            Candidate { persistent: p, volatile, internal } => Candidate {
                persistent: p.incr_term(),
                volatile,
                internal,
            },
            Follower { persistent: p, volatile, internal } => Follower {
                persistent: p.incr_term(),
                volatile,
                internal,
            },
            Leader { .. } => panic!("leader becoming candidate!"),
        };
        StateMachine { state: state }
    }

    fn become_leader(self) -> StateMachine {
        let state = match self.state {
            Candidate { persistent, volatile, internal } => {
                let last_applied = volatile.last_applied;
                // TODO: where do we find the server num?
                let leader = LeaderVolatileState::new(last_applied, 2);
                Leader { persistent, volatile, leader, internal }
            },
            Leader { .. } => panic!("leader becoming leader!"),
            Follower { .. } => panic!("follower becoming leader!"),
        };
        StateMachine { state }
    }

    fn become_follower(self, term: TermId) -> StateMachine {
        let StateMachine { state } = self;
        let state = match state {
            Leader { persistent, volatile, leader: _, internal } => {
                let persistent = persistent.with_new_term(term);
                Follower { persistent, volatile, internal }
            },
            Candidate { persistent, volatile, internal } => {
                let persistent = persistent.with_new_term(term);
                Follower { persistent, volatile, internal }
            },
            Follower { persistent, volatile, internal } => {
                let persistent = persistent.with_new_term(term);
                Follower { persistent, volatile, internal }
            },
        };
        StateMachine { state }
    }
}
