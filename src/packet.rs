use alloc::vec::Vec;

use crate::prelude::NodeId;

pub struct PacketHeader<N: NodeId> {
    pub id: u64,
    pub from: N,
    pub to: N,
}

pub struct FindNode<N: NodeId> {
    pub target: N,
}

pub struct NeighborInfo<N: NodeId, I> {
    pub nodeid: N,
    pub network_info: I,
}

pub enum PacketBody<N: NodeId, I> {
    Ping,
    Pong,
    FindNode(FindNode<N>),
    NeighborInfo(Vec<NeighborInfo<N, I>>),
}

pub struct Packet<N: NodeId, I> {
    pub header: PacketHeader<N>,
    pub body: PacketBody<N, I>,
}
