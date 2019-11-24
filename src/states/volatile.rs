pub type LogEntryIndex = u64;
const LOG_ENTRY_INDEX_ZERO: LogEntryIndex = 0;

/// volatile state for all servers in Raft cluster.
pub struct ServerVolatileState {
    /// index of highest log entry known to be committed
    /// (initialized to 0, increases monotonically)
    pub commit_index: LogEntryIndex,

    /// index of highest log entry applied to state machine
    /// (initialized to 0, increases monotonically)
    pub last_applied: LogEntryIndex,
}

/// volatile state for leader
pub struct LeaderVolatileState {
    /// for each server, index of next log entry to send to that server
    /// (initialized to leader last log index + 1)
    pub next_index: Vec<LogEntryIndex>,

    /// for each server, index of highest log entry known to be replicated
    /// on that server (initialized to 0, increases monotonically)
    pub match_index: Vec<LogEntryIndex>,
}

impl ServerVolatileState {
    /// create an empty instance
    pub fn new() -> ServerVolatileState {
        ServerVolatileState {
            commit_index: LOG_ENTRY_INDEX_ZERO,
            last_applied: LOG_ENTRY_INDEX_ZERO,
        }
    }
}

impl LeaderVolatileState {
    /// create a new instance with last applied log index from the leader and
    /// the number of servers in the cluster.
    pub fn new(last_applied: LogEntryIndex, num_servers: usize) -> LeaderVolatileState {
        LeaderVolatileState {
            next_index:  vec![last_applied + 1; num_servers],
            match_index: vec![LOG_ENTRY_INDEX_ZERO; num_servers],
        }
    }
}
