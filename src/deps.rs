/* 
 Handling of dependency managment including
 installation, updating, uninstalling etc... 
*/

#[derive(Debug, Default)]
pub struct DepManager {
    dependencies: Vec<Dependency>
}

#[derive(Debug)]
pub struct Dependency {
    name: String,
    version: f32,
    origin: String,
}

impl Dependency {
    pub fn new(name: String, origin: String, version: f32) -> Self {
        Self { name: name.to_string(), version, origin }
    }
}

impl DepManager {
    pub fn new(dependencies: Vec<Dependency>) -> Self {
        Self { dependencies }
    }
}