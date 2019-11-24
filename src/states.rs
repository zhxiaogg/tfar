mod persistent;
mod state_machine;
mod volatile;

pub use volatile::{LeaderVolatileState, LogEntryIndex, ServerVolatileState};

pub use persistent::{CandidateId, Command, LogEntry, PersistentState, TermId};

/// The three states for servers in Raft cluter
pub enum ServerState {
    /// The one and only leader in cluster
    Leader {
        persistent: PersistentState,
        volatile:   ServerVolatileState,
        leader:     LeaderVolatileState,
    },
    /// A server with an ongoing election
    Candidate { persistent: PersistentState, volatile: ServerVolatileState },
    /// A server waiting for requests from other servers(vote, append entries
    /// and snapshot)
    Follower { persistent: PersistentState, volatile: ServerVolatileState },
}
