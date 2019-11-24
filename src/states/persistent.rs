/// In Raft, time are devided into terms, they are in arbitrary length,
/// and there will be at most one leader in each term. Terms are consecutive
/// numbers, and act as a logical clock in Raft.
pub type TermId = u64;

pub type CandidateId = u64;

/// A command to be applied on state machines
pub struct Command {}

pub struct LogEntry {
    /// Command to be applied on state machine
    command: Command,
    /// The term this log entry belongs to. None empty when this entry
    /// was received by leader.
    term: Option<TermId>,
}

/// persistent state on all servers in Raft cluster.
pub struct PersistentState {
    /// Latest term server has seen (initialized to 0 on first boot,
    /// increases monotonically)
    current_term: TermId,
    /// CandidateId that received vote in current term (can be empty)
    voted_for: Option<CandidateId>,
    /// Log entries
    log: Vec<LogEntry>,
}

impl PersistentState {
    pub fn new() -> PersistentState {
        PersistentState {
            current_term: 0,
            voted_for: Option::None,
            log: Vec::new(),
        }
    }
}
