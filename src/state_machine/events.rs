use super::states::{LogEntry, LogEntryId, LogEntryIndex, ServerId, TermId};
use std::time::Duration;

pub enum StateEvent {
    Timeout(Duration),
    VoteRequest {
        /// candidate's term
        term: TermId,
        /// candidate requesting vote
        candidate: ServerId,
        /// id of candidate's last log entry
        last_log: LogEntryId,
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
        leader: ServerId,
        /// id of log entry immediately precedding new entries
        prev_log: LogEntryId,
        /// new entries
        entries: Vec<LogEntry>,
        /// leader's commit index
        commit_idx: LogEntryIndex,
    },
    AppendEntriesResponse {
        /// current Term, for leader to update itself
        term: TermId,
        /// true if follower contained entry matching prevLogIndex and prefLogTerm in request
        success: bool,
    },
}
