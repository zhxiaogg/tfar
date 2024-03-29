use super::{LogEntryId, LogEntryIndex, ServerId};
use std::clone::Clone;

/// In Raft, time are devided into terms, they are in arbitrary length,
/// and there will be at most one leader in each term. Terms are consecutive
/// numbers, and act as a logical clock in Raft.
pub type TermId = u64;

/// A command to be applied on state machines
pub enum Command {
    /// tfar internal commands
    Tfar {},
    /// user defined commands
    Client(Vec<u8>),
}

#[derive(Clone)]
pub struct LogEntry {
    /// The term this log entry belongs to. None empty when this entry
    /// was received by leader.
    pub term: Option<TermId>,
    pub index: LogEntryIndex,
}

/// persistent state on all servers in Raft cluster.
pub struct PersistentState {
    /// Latest term server has seen (initialized to 0 on first boot,
    /// increases monotonically)
    current_term: TermId,
    /// CandidateId that received vote in current term (can be empty)
    voted_for: Option<ServerId>,
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

    pub fn accept_candidate(&self, candidate: ServerId) -> bool {
        match self.voted_for {
            None => true,
            Some(c) if c == candidate => true,
            _ => false,
        }
    }

    pub fn accept_log(&self, last_log: &LogEntryId) -> bool {
        if let Some(log) = self.log.last() {
            last_log.term >= log.term.unwrap_or(0) && last_log.index >= log.index
        } else {
            true
        }
    }

    pub fn accept_term(&self, term: TermId) -> bool {
        self.current_term < term
    }

    /// create a new PersistentState for a vote
    pub fn with_vote(self, term: TermId, candidate: ServerId) -> PersistentState {
        PersistentState {
            current_term: term,
            voted_for:    Some(candidate),
            log:          self.log,
        }
    }

    /// TOOD: append log entries according to preceding log entry info
    pub fn with_log_entries(self, prev_log: LogEntryId, entries: Vec<LogEntry>) -> PersistentState {
        let mut new_entries = Vec::new();
        new_entries.extend(entries.iter().cloned());
        new_entries.extend(self.log.iter().cloned());
        PersistentState {
            current_term: self.current_term,
            voted_for:    self.voted_for,
            log:          new_entries,
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

    pub fn with_vote_for(self, candidate: ServerId) -> PersistentState {
        PersistentState {
            current_term: self.current_term,
            voted_for:    Some(candidate),
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

impl LogEntry {
    pub fn zero() -> LogEntry {
        LogEntry { term: Some(0), index: 0 }
    }
}
