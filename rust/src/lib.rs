pub mod adapters;
pub mod dependency_resolver;
pub mod download_manager;
pub mod environment_manager;
pub mod config;
pub mod package;

pub use adapters::RepositoryAdapter;
pub use dependency_resolver::DependencyResolver;
pub use download_manager::DownloadManager;
pub use environment_manager::EnvironmentManager;
pub use config::ConfigManager;
pub use package::{PackageMetadata, DependencyRelation};