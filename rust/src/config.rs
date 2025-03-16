use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub repositories: Vec<RepositoryConfig>,
    pub install_path: PathBuf,
    pub cache_path: PathBuf,
    pub max_parallel_downloads: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepositoryConfig {
    pub name: String,
    pub adapter: String,
    pub url: String,
    pub priority: u32,
    pub enabled: bool,
}

pub struct ConfigManager {
    config_path: PathBuf,
    config: Config,
}

impl ConfigManager {
    pub fn new(home_dir: &std::path::Path) -> Result<Self, std::io::Error> {
        let config_path = home_dir.join(".ngm").join("config.toml");
        let config_content = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&config_content)?;
        
        Ok(ConfigManager {
            config_path,
            config,
        })
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let toml_content = toml::to_string(&self.config)?;
        std::fs::write(&self.config_path, toml_content)?;
        Ok(())
    }

    pub fn get_repository_adapters(&self) -> Vec<Box<dyn RepositoryAdapter>> {
        self.config.repositories
            .iter()
            .filter(|r| r.enabled)
            .sorted_by(|a, b| a.priority.cmp(&b.priority))
            .map(|r| self.create_adapter(r))
            .collect()
    }

    fn create_adapter(&self, config: &RepositoryConfig) -> Box<dyn RepositoryAdapter> {
        match config.adapter.as_str() {
            "pypi" => Box::new(super::adapters::pypi::PyPIAdapter::new()),
            "npm" => Box::new(super::adapters::npm::NpmAdapter::new()),
            "apt" => Box::new(super::adapters::apt::AptAdapter::new()),
            _ => panic!("Unsupported repository adapter: {}", config.adapter),
        }
    }
}