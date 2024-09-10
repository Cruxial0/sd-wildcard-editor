use std::{fs::{self, create_dir, metadata}, path::{Path, PathBuf}};

pub struct DeployNode {
    lines: Vec<String>,
    rel_path: PathBuf,
    children: Vec<DeployNode>
}

impl DeployNode {
    pub fn new(lines: Vec<String>, rel_path: impl AsRef<Path>, children: Vec<DeployNode>) -> DeployNode {
        DeployNode {
            lines: lines,
            rel_path: rel_path.as_ref().to_path_buf(),
            children: children,
        }
    }

    pub fn deploy_recursive(&self, deploy_dir: impl AsRef<Path>) {
        self.deploy(&deploy_dir);
        self.deploy_children(&deploy_dir);
    }

    pub fn deploy(&self, deploy_dir: impl AsRef<Path>) {
        let path = deploy_dir.as_ref().join(self.rel_path.clone());
        if (!path.exists() || path.is_dir()) && self.lines.len() == 0 {
            create_dir(path);
            return;
        } 
        
        if let Some(ext) = path.extension() {
            fs::write(path, self.lines.join("\n"));
            return;
        }

        println!("Encountered a Node that was neither a directory or a file: {}", path.to_str().unwrap());
    }

    fn deploy_children(&self, deploy_dir: impl AsRef<Path>) {
        let dir = deploy_dir;
        for child in &self.children {
            child.deploy(&dir);
        }
    }
}