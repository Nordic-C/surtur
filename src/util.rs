pub fn root_dir_name(cur_dir: &str) -> &str {
    let dirs: Vec<&str> = cur_dir.split("\\").collect();
    dirs[dirs.len()-1]
}