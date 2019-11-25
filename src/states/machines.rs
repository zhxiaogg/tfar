use async_trait::async_trait;
mod follower;

use super::events::StateEvent;
pub use follower::Follower;

#[async_trait]
pub trait StateMachine {
    async fn on_events(self, event: StateEvent) -> Box<dyn StateMachine>;
}
