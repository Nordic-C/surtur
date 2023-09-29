/* 
 Handling of dependency managment including
 installation, updating, uninstalling etc... 
*/

pub struct DepManager {
    dependencies: Vec<Dependency>
}

pub struct Dependency {
    name: String,
    version: f32,
}

impl DepManager {
    pub fn new(dependecies: Vec<Dependency>) {
        todo!()
    }
}