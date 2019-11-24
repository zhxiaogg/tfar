use super::{
    persistent::{PersistentState, TermId},
    volatile::{LeaderVolatileState, ServerVolatileState},
    ServerState,
    ServerState::*,
};

/// StateMachine contains the state transition for Raft servers described 
/// by the original paper in `Figure 4: Server states`
struct StateMachine {
    state: ServerState,
}

enum StateEvent {
    Timeout(),
    WonElection(),
    NewLeader { term: TermId },
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            state: Follower {
                persistent: PersistentState::new(),
                volatile:   ServerVolatileState::new(),
            },
        }
    }
    pub fn on_message(self, msg: StateEvent) -> StateMachine {
        use StateEvent::*;
        match msg {
            Timeout() => self.become_candidate(),
            WonElection() => self.become_leader(),
            NewLeader { term } => self.become_follower(term),
        }
    }

    fn become_candidate(self) -> StateMachine {
        let state = match self.state {
            Candidate { persistent: p, volatile } => Candidate { persistent: p.incr_term(), volatile },
            Follower { persistent: p, volatile } => Follower { persistent: p.incr_term(), volatile },
            Leader { persistent: _, volatile: _, leader: _ } => panic!("leader becoming candidate!"),
        };
        StateMachine { state: state }
    }

    fn become_leader(self) -> StateMachine {
        let state = match self.state {
            Candidate { persistent, volatile } => {
                let last_applied = volatile.last_applied;
                // TODO: where do we find the server num?
                let leader = LeaderVolatileState::new(last_applied, 2);
                Leader { persistent, volatile, leader }
            },
            Leader { persistent: _, volatile: _, leader: _ } => panic!("leader becoming leader!"),
            Follower { persistent: _, volatile: _ } => panic!("follower becoming leader!"),
        };
        StateMachine { state }
    }

    fn become_follower(self, term: TermId) -> StateMachine {
        let StateMachine { state } = self;
        let state = match state {
            Leader { persistent, volatile, leader: _ } => {
                let persistent = persistent.with_new_term(term);
                Follower { persistent, volatile }
            },
            Candidate { persistent, volatile } => {
                let persistent = persistent.with_new_term(term);
                Follower { persistent, volatile }
            },
            Follower { persistent, volatile } => {
                let persistent = persistent.with_new_term(term);
                Follower { persistent, volatile }
            },
        };
        StateMachine { state }
    }
}
