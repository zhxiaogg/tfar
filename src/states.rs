mod internal;
mod persistent;
mod state_machine;
mod volatile;

pub use volatile::{LeaderVolatileState, LogEntryIndex, ServerVolatileState};

pub use persistent::{CandidateId, Command, LogEntry, PersistentState, TermId};

pub use internal::{InternalState, Server, ServerId, VoteResult};

/// The three states for servers in Raft cluter
pub enum ServerState {
    /// The one and only leader in cluster
    Leader {
        persistent: PersistentState,
        volatile:   ServerVolatileState,
        leader:     LeaderVolatileState,
        internal:   InternalState,
    },
    /// A server with an ongoing election
    Candidate {
        persistent: PersistentState,
        volatile:   ServerVolatileState,
        internal:   InternalState,
    },
    /// A server waiting for requests from other servers(vote, append entries
    /// and snapshot)
    Follower {
        persistent: PersistentState,
        volatile:   ServerVolatileState,
        internal:   InternalState,
    },
}

impl ServerState {
    pub fn term(&self) -> TermId {
        use ServerState::*;
        match self {
            Leader { persistent, .. } => persistent.term(),
            Candidate { persistent, .. } => persistent.term(),
            Follower { persistent, .. } => persistent.term(),
        }
    }
}
