use async_trait::async_trait;
pub mod follower;

use super::events::StateEvent;

#[async_trait]
pub trait StateMachine {
    async fn on_events(self, event: StateEvent) -> Box<dyn StateMachine>;
}
