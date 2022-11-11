use crate::router::HandlerFn;

pub struct Node {
    pub nodes: Vec<Node>,
    pub key: String,
    pub handler: Option<HandlerFn>,
}

impl Node {
    pub fn new(key: &str) -> Self {
        Self {
            nodes: Vec::new(),
            key: String::from(key),
            handler: None,
        }
    }

    pub fn insert(&mut self, path: &str, f: HandlerFn) {
        match path.split_once('/') {
            Some((root, "")) => {
                self.key = String::from(path);
                self.handler = Some(f);
            }
            Some(("", path)) => self.insert(path, f),
            Some((root, path)) => {
                let node = self.nodes.iter_mut().find(|v| root == &v.key);
                match node {
                    Some(n) => n.insert(path, f),
                    None => {
                        let mut node = Node::new(root);
                        node.insert(path, f);
                        self.nodes.push(node);
                    }
                }
            }
            None => {
                let mut node = Node::new(path);
                node.handler = Some(f);
                self.nodes.push(node);
            }
        }
    }

    pub fn get(&self, path: &str) -> Option<HandlerFn> {
        match path.split_once('/') {
            Some((root, "")) => {
                if root == &self.key {
                    self.handler
                } else {
                    None
                }
            },
            Some(("", path)) => self.get(path),
            Some((root, path)) => {
                let node = self.nodes.iter().find(|m| root == &m.key);
                if let Some(node) = node {
                    node.get(path)
                } else {
                    None
                }
            },
            None => {
                let node = self.nodes.iter().find(|m| path == &m.key);
                if let Some(node) = node {
                    node.handler
                } else {
                    None
                }
            }
        }
    }
}
