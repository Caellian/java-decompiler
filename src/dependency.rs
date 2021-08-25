use std::path::PathBuf;

#[derive(Clone)]
pub enum Dependency {
    MavenDependency {
        namespace: String,
        name: String,
        version: String,
    },
    Jar {
        path: PathBuf,
    },
    Remote {
        url: String,
    },
}

pub struct DependencyInfo {
    pub dependency: Dependency,

    pub classes: Vec<String>,
}
