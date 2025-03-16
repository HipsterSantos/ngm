use super::RepositoryAdapter;
use crate::{
    PackageMetadata,
    RepoError,
    DependencyRelation,
};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct NpmResponse {
    name: String,
    versions: std::collections::HashMap<String, NpmVersion>,
    "dist-tags": NpmDistTags,
}

#[derive(Deserialize, Debug)]
struct NpmVersion {
    dependencies: std::collections::HashMap<String, String>,
    dist: NpmDist,
}

#[derive(Deserialize, Debug)]
struct NpmDist {
    tarball: String,
}

#[derive(Deserialize, Debug)]
struct NpmDistTags {
    latest: String,
}

pub struct NpmAdapter {
    client: Client,
}

impl NpmAdapter {
    pub fn new() -> Self {
        NpmAdapter {
            client: Client::new(),
        }
    }
}

impl RepositoryAdapter for NpmAdapter {
    fn search(&self, query: &str) -> Result<Vec<PackageMetadata>, RepoError> {
        let url = format!("https://registry.npmjs.org/-/v1/search?text={}", query);
        let response = self.client.get(&url).send().map_err(|_| RepoError::SearchFailed)?;
        // Parse response and convert to PackageMetadata
        Ok(vec![])
    }

    fn fetch_metadata(&self, package_name: &str) -> Result<PackageMetadata, RepoError> {
        let url = format!("https://registry.npmjs.org/{}", package_name);
        let response = self.client.get(&url).send().map_err(|_| RepoError::NotFound)?;
        let npm_response: NpmResponse = response.json().map_err(|_| RepoError::NotFound)?;
        
        let version = npm_response."dist-tags".latest;
        let version_data = npm_response.versions.get(&version).ok_or(RepoError::NotFound)?;
        
        let mut dependencies = Vec::new();
        for (name, version_req) in &version_data.dependencies {
            dependencies.push(format!("{}@{}", name, version_req));
        }

        let download_url = version_data.dist.tarball.clone();

        Ok(PackageMetadata {
            name: npm_response.name,
            version,
            dependencies,
            download_url: Some(download_url),
            repository: "npm".to_string(),
        })
    }

    fn download_package(&self, package: &PackageMetadata, dest: &Path) -> Result<(), RepoError> {
        if let Some(url) = &package.download_url {
            let response = self.client.get(url).send().map_err(|_| RepoError::DownloadFailed)?;
            let content = response.bytes().map_err(|_| RepoError::DownloadFailed)?;
            std::fs::write(dest, content).map_err(|_| RepoError::DownloadFailed)?;
            Ok(())
        } else {
            Err(RepoError::NoDownloadUrl)
        }
    }
}