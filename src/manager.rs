/* 
 Handling of dependency managment including
 installation, updating, uninstalling etc... 
*/

pub struct DepManager {
    dependencies: Vec<Dependency>
}

#[derive(Debug)]
pub struct Dependency {
    name: String,
    version: f32,
}

impl Dependency {
    pub fn new(name: String, version: f32) -> Self {
        Self { name: name.to_string(), version }
    }
}

impl DepManager {
    pub fn new(dependecies: Vec<Dependency>) {
        todo!()
    }
}