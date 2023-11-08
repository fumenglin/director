use crate::core::node::node::{Node, NodeRunResult};

pub mod node;

pub trait NodeFace {
    fn run(&self) -> NodeRunResult;
    fn get_next(&self) -> Node;
}

