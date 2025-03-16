pub mod pypi;
pub mod npm;
pub mod apt;

pub trait RepositoryAdapter {
    fn search(&self, query: &str) -> Result<Vec<PackageMetadata>, RepoError>;
    fn fetch_metadata(&self, package_name: &str) -> Result<PackageMetadata, RepoError>;
    fn download_package(&self, package: &PackageMetadata, dest: &std::path::Path) -> Result<(), RepoError>;
}

#[derive(Debug)]
pub enum RepoError {
    SearchFailed,
    NotFound,
    NoDownloadUrl,
    DownloadFailed,
}