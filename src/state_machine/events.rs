use super::states::{LogEntry, LogEntryIndex, ServerId, TermId};
use std::time::Duration;

pub enum StateEvent {
    Timeout(Duration),
    VoteRequest {
        /// candidate's term
        term: TermId,
        /// candidate requesting vote
        candidate_id: ServerId,
        /// index of candidate's last log entry
        last_log_index: LogEntryIndex,
        /// term of candidate's last log entry
        last_log_term: TermId,
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
        leader_id: ServerId,
        /// index of log entry immediately precedding new entries
        prev_log_index: LogEntryIndex,
        /// term of log entry immediately precedding new entries
        prev_log_term: TermId,
        /// new entries
        entries: Vec<LogEntry>,
        /// leader's commit index
        leader_commit: LogEntryIndex,
    },
    AppendEntriesResponse {
        /// current Term, for leader to update itself
        term: TermId,
        /// true if follower contained entry matching prevLogIndex and prefLogTerm in request
        success: bool,
    },
}
