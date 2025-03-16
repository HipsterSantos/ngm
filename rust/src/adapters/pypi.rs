use crate::{
    RepositoryAdapter,
    PackageMetadata,
    DependencyRelation,
    RepoError,
};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct PyPIResponse {
    info: PyPIInfo,
    urls: Vec<PyPIUrl>,
}

#[derive(Deserialize, Debug)]
struct PyPIInfo {
    name: String,
    version: String,
    requires_dist: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct PyPIUrl {
    url: String,
    packagetype: String,
}

pub struct PyPIAdapter {
    client: Client,
}

impl PyPIAdapter {
    pub fn new() -> Self {
        PyPIAdapter {
            client: Client::new(),
        }
    }
}

impl RepositoryAdapter for PyPIAdapter {
    fn search(&self, query: &str) -> Result<Vec<PackageMetadata>, RepoError> {
        let url = format!("https://pypi.org/pypi?pypi-search?:action=search&term={}", query);
        let response = self.client.get(&url).send()?;
        // Parse response and convert to PackageMetadata
        Ok(vec![])
    }

    fn fetch_metadata(&self, package_name: &str) -> Result<PackageMetadata, RepoError> {
        let url = format!("https://pypi.org/pypi/{}/json", package_name);
        let response = self.client.get(&url).send()?;
        let pypi_response: PyPIResponse = response.json()?;
        
        let mut dependencies = Vec::new();
        for req in pypi_response.info.requires_dist {
            dependencies.push(req.parse()?);
        }

        let download_url = pypi_response.urls
            .iter()
            .find(|url| url.packagetype == "sdist")
            .map(|url| url.url.clone());

        Ok(PackageMetadata {
            name: pypi_response.info.name,
            version: pypi_response.info.version,
            dependencies,
            download_url,
            repository: "pypi".to_string(),
        })
    }

    fn download_package(&self, package: &PackageMetadata, dest: &Path) -> Result<(), RepoError> {
        if let Some(url) = &package.download_url {
            let response = self.client.get(url).send()?;
            let content = response.bytes()?;
            std::fs::write(dest, content)?;
            Ok(())
        } else {
            Err(RepoError::NoDownloadUrl)
        }
    }
}