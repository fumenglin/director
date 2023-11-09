use std::collections::HashMap;
use crate::core::node::node::Node;

pub struct Playbook {
    pub paths: HashMap<String, Vec<Node>>,
    pub start_node: Node,
}


impl Playbook {
    pub fn new(paths: HashMap<String, Vec<None>>, start_node: Node) -> Playbook {
        //获取所有的路径
        Playbook {
            paths,
            start_node,
        }
    }

    pub fn get_start_node(&self) -> Option<Vec<Node>> {
        //查找开始点，如果没有则报错
        let start_id = &self.start_node.uid;
        let start_node = &self.paths.get(start_id);
        if start_node.is_none() {
            return None;
        }
        return start_node.cloned();
    }

    pub fn run(&self) -> Result<(),  Box<dyn std::error::Error>> {
        //先看是否存在开始接点
        let start_node = self.get_start_node();
        if start_node.is_none() {
            return Err(Box::new(()));
        }

        return Ok(());
    }

    pub fn  dfs(&self)-> Result<(),Err()>{

    }
}