import toml
import os
from pathlib import Path

def load_core():
    # This would normally load the Rust core component
    # For this demo, we'll just return a mock object
    class MockCore:
        def resolve_dependencies(self, package):
            return [PackageMetadata("dependency1", "1.0.0", [], None, "pypi")]
        
        def search_packages(self, query, repo):
            return [{"name": "package1", "version": "1.0.0", "repository": "pypi"}]
        
        def prepare_downloads(self, deps):
            return [{"id": "1", "package_name": "package1", "size": 1024 * 1024}]
        
        def install_packages(self, deps):
            pass
        
        def update_package(self, package):
            pass
        
        def update_all_packages(self):
            pass
        
        def remove_package(self, package):
            pass
        
        def list_installed_packages(self):
            return [{"name": "package1", "version": "1.0.0"}]
    
    return MockCore()

class PackageMetadata:
    def __init__(self, name, version, dependencies, download_url, repository):
        self.name = name
        self.version = version
        self.dependencies = dependencies
        self.download_url = download_url
        self.repository = repository