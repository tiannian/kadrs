use crate::Packet;

use alloc::boxed::Box;

use super::NodeId;

#[async_trait::async_trait]
pub trait Transport<N: NodeId, I> {
    async fn send(&mut self, req: &Packet<N, I>) -> Packet<N, I>;
}
