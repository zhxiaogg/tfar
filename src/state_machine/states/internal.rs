use std::collections::HashSet;

/// Represents the server id
pub type ServerId = usize;

/// Represents a node in Raft cluster
pub struct Server {
    address: String,
    port:    u16,
}

/// Tracing the progress of an ongoing Vote event.
struct Voting {
    agrees:  HashSet<ServerId>,
    rejects: HashSet<ServerId>,
}

pub enum VoteResult {
    Agreed(usize),
    Rejected(usize),
    NotYet(usize),
}

/// State for tfar internal implementations. Some of them are persistent, while some of them are volatile.
pub struct InternalState {
    /// list of servers in Raft cluster
    servers: Vec<Server>,
    /// A possibly ongoing Vot event. Should be non-emtpy for candidate server.
    voting: Option<Voting>,
}

impl InternalState {
    pub fn new() -> InternalState {
        InternalState { servers: Vec::new(), voting: None }
    }

    pub fn clear_voting(self) -> InternalState {
        InternalState { servers: self.servers, voting: None }
    }

    /// creates a new states with a vote response
    pub fn with_vote(self, server: ServerId, granted: bool) -> InternalState {
        match self {
            InternalState { servers: _, voting: None } => panic!("there is no ongoing vote!"),
            InternalState { servers, voting: Some(voting) } => InternalState {
                servers,
                voting: Some(voting.accept_vote(server, granted)),
            },
        }
    }

    /// Check if an ongoint voting gets granted.
    pub fn vote_granted(&self) -> VoteResult {
        match self {
            InternalState { servers: _, voting: None } => panic!("there is no ongoing vote!"),
            InternalState { servers, voting: Some(ref voting) } => voting.vote_granted(servers.len()),
        }
    }
}

impl Voting {
    /// create a new Voting by updating current one with an vote result
    fn accept_vote(self, server: ServerId, agree: bool) -> Voting {
        let Voting { mut agrees, mut rejects } = self;
        if agree && agrees.contains(&server) || !agree && rejects.contains(&server) {
            eprintln!("server {} has voted!", server);
        } else if agree {
            agrees.insert(server);
        } else {
            rejects.insert(server);
        }
        Voting { agrees, rejects }
    }

    /// check whether or or this voting has been granted
    fn vote_granted(&self, num_servers: usize) -> VoteResult {
        let num_valid = num_servers / 2 + 1;
        let agrees = self.agrees.len();
        let rejects = self.rejects.len();

        if agrees >= num_valid {
            VoteResult::Agreed(agrees)
        } else if rejects >= num_valid {
            VoteResult::Rejected(rejects)
        } else {
            VoteResult::NotYet(agrees + rejects)
        }
    }
}
