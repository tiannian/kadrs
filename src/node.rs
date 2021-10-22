use core::marker::PhantomData;

use crate::prelude::{NodeId, Store, Transport};

pub struct Node<N: NodeId, S: Store, I, T: Transport<N, I>> {
    pub id: N,
    pub store: S,
    pub transport: T,
    pub marker: PhantomData<I>,
}

