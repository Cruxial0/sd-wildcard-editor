use std::path::Path;

use tauri::AppHandle;

use super::deploy_node::DeployNode;

pub trait Deployable {
    fn generate_deploy_node(&self, path: impl AsRef<Path>, handle: &AppHandle) -> Option<DeployNode>;
}