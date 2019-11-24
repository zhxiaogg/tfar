/// In Raft, time are devided into terms, they are in arbitrary length,
/// and there will be at most one leader in each term. Terms are consecutive
/// numbers, and act as a logical clock in Raft.
pub type TermId = u64;

pub type CandidateId = u64;

/// A command to be applied on state machines
pub enum Command {
    /// tfar internal commands
    Tfar {},
    /// user defined commands
    Client(Vec<u8>),
}

pub struct LogEntry {
    /// Command to be applied on state machine
    pub command: Command,
    /// The term this log entry belongs to. None empty when this entry
    /// was received by leader.
    pub term: Option<TermId>,
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
            voted_for:    Option::None,
            log:          Vec::new(),
        }
    }

    pub fn term(&self) -> TermId {
        self.current_term
    }

    /// create a new PersistentState for a vote
    pub fn with_vote(self, term: TermId, candidate: CandidateId) -> PersistentState {
        PersistentState {
            current_term: term,
            voted_for:    Some(candidate),
            log:          self.log,
        }
    }

    /// Create a new PersistentState by cloning current state with a new term
    pub fn with_new_term(self, term: TermId) -> PersistentState {
        PersistentState {
            current_term: term,
            voted_for:    None,
            log:          self.log,
        }
    }
    /// create a new PersistentState by increase term
    pub fn incr_term(self) -> PersistentState {
        PersistentState {
            current_term: self.current_term + 1,
            voted_for:    None,
            log:          self.log,
        }
    }
}
