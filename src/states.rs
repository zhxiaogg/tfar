mod persistent;
mod volatile;

pub use volatile::LeaderVolatileState;
pub use volatile::LogEntryIndex;
pub use volatile::ServerVolatileState;

pub use persistent::CandidateId;
pub use persistent::Command;
pub use persistent::LogEntry;
pub use persistent::PersistentState;
pub use persistent::TermId;

/// The three states for servers in Raft cluter
pub enum ServerState {
    /// The one and only leader in cluster
    Leader,
    /// A server with an ongoing election
    Candidate,
    /// A server waiting for requests from other servers(vote, append entries 
    /// and snapshot)
    Follower,
}
