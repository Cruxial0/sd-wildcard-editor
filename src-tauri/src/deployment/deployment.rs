use std::path::{Path, PathBuf};

use crate::helpers::dir_utils::{get_or_create_path, get_public_directory};

use super::deploy_node::DeployNode;

pub struct Deployment {
    base_path: PathBuf,
    nodes: Vec<DeployNode>
}

impl Deployment {
    pub fn new(base_path: impl AsRef<Path>) -> Deployment {
        Deployment {
            base_path: base_path.as_ref().to_path_buf(),
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: DeployNode) {
        self.nodes.push(node);
    }

    pub fn deploy(&self) {
        let public_dir = get_public_directory();

        let dir = get_or_create_path(self.base_path.clone()).unwrap();

        for node in &self.nodes {
            node.deploy_recursive(&dir);
        }
    }
}