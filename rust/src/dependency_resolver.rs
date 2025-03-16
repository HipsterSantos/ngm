use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo;
use crate::{
    RepositoryAdapter,
    PackageMetadata,
    RepoError,
};
use std::collections::HashMap;

#[derive(Debug)]
pub enum ResolveError {
    CyclicDependencies,
    PackageNotFound(String),
    VersionConflict(String),
}

pub struct DependencyResolver {
    adapters: Vec<Box<dyn RepositoryAdapter>>,
}

impl DependencyResolver {
    pub fn new(adapters: Vec<Box<dyn RepositoryAdapter>>) -> Self {
        DependencyResolver { adapters }
    }

    pub fn resolve_dependencies(&self, root_package: &str) -> Result<Vec<PackageMetadata>, ResolveError> {
        let mut graph = Graph::new();
        let root_node = self.add_package_to_graph(&mut graph, root_package)?;

        self.build_dependency_graph(&mut graph, root_node)?;
        self.resolve_conflicts(&graph)?;

        let installation_order = self.topological_sort(graph)?;
        Ok(installation_order)
    }

    fn add_package_to_graph(&self, graph: &mut Graph<PackageMetadata, DependencyRelation>, package_name: &str) -> Result<NodeIndex, ResolveError> {
        for adapter in &self.adapters {
            match adapter.fetch_metadata(package_name) {
                Ok(metadata) => {
                    let node = graph.add_node(metadata);
                    return Ok(node);
                },
                Err(RepoError::NotFound) => continue,
                Err(e) => return Err(ResolveError::from(e)),
            }
        }
        Err(ResolveError::PackageNotFound(package_name.to_string()))
    }

    fn build_dependency_graph(&self, graph: &mut Graph<PackageMetadata, DependencyRelation>, node: NodeIndex) -> Result<(), ResolveError> {
        let package = &graph[node];
        for dependency in &package.dependencies {
            let dep_node = self.add_package_to_graph(graph, &dependency)?;
            graph.add_edge(node, dep_node, DependencyRelation::Direct);
            self.build_dependency_graph(graph, dep_node)?;
        }
        Ok(())
    }

    fn resolve_conflicts(&self, graph: &Graph<PackageMetadata, DependencyRelation>) -> Result<(), ResolveError> {
        // Implement conflict resolution logic
        Ok(())
    }

    fn topological_sort(&self, graph: Graph<PackageMetadata, DependencyRelation>) -> Result<Vec<PackageMetadata>, ResolveError> {
        match algo::toposort(&graph, None) {
            Ok(order) => Ok(order.into_iter().map(|node| graph[node].clone()).collect()),
            Err(_) => Err(ResolveError::CyclicDependencies),
        }
    }
}

impl From<RepoError> for ResolveError {
    fn from(error: RepoError) -> Self {
        match error {
            RepoError::NotFound(pkg) => ResolveError::PackageNotFound(pkg),
            RepoError::VersionConflict(pkg) => ResolveError::VersionConflict(pkg),
            _ => ResolveError::VersionConflict("Unknown".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DependencyRelation {
    Direct,
    Transitive,
}