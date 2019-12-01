mod internal;
mod persistent;
mod volatile;

pub use volatile::{LeaderVolatileState, ServerVolatileState};

pub use persistent::{Command, LogEntry, PersistentState, TermId};

pub use internal::{InternalState, Server, VoteResult};

pub type LogEntryIndex = u64;

/// Represents the server id
pub type ServerId = usize;

/// Identifier for a log entry by index and term
pub struct LogEntryId {
    /// index of this log entry
    pub index: LogEntryIndex,
    /// term of this log entry
    pub term: TermId,
}
