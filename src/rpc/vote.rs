use crate::states::{CandidateId, LogEntryIndex, TermId};
use async_trait::async_trait;

/// Vote request.
pub struct VoteRequest {
    /// candidate's term
    pub term: TermId,
    /// candidate requesting vote
    pub candidate_id: CandidateId,
    /// index of candidate's last log entry
    pub last_log_index: LogEntryIndex,
    /// term of candidate's last log entry
    pub last_log_term: TermId,
}

/// Vote response
pub struct VoteResponse {
    /// Current Term from requested server, for candidate to update itself
    pub term: TermId,
    /// true means candidate received vote
    pub vote_granted: bool,
}

/// Vote rpc service definition
#[async_trait]
pub trait VoteService {
    /// receive VoteRequest from candidates and return a VoteResponse according
    /// to Raft specification
    async fn request_vote(&mut self, request: VoteRequest) -> VoteResponse;
}

/// Vote rpc client definition
#[async_trait]
pub trait VoteClient {
    /// Sending a VoteRequest to a remote server
    async fn request_vote(&self, request: VoteRequest) -> VoteResponse;
}
