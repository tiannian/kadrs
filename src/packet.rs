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

pub struct NeighborInfo<N: NodeId> {
    pub neighbors: Vec<N>,
}

pub enum PacketBody<N: NodeId> {
    Ping,
    Pong,
    FindNode(FindNode<N>),
    NeighborInfo(NeighborInfo<N>),
}

pub struct Packet<N: NodeId> {
    pub header: PacketHeader<N>,
    pub body: PacketBody<N>,
}
