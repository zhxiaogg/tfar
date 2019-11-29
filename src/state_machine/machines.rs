use async_trait::async_trait;
pub mod candidate;
pub mod follower;
pub mod leader;

use super::events::StateEvent;

#[async_trait]
pub trait StateMachine {
    async fn on_events(self, event: StateEvent) -> Box<dyn StateMachine>;
}
