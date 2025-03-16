use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub download_url: Option<String>,
    pub repository: String,
}

impl PackageMetadata {
    pub fn bin_directory(&self) -> Option<PathBuf> {
        // Determine binary directory based on package type
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DependencyRelation {
    Direct,
    Transitive,
}