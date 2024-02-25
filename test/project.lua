-- properties
Name = "test"
Props = {
    std = "c17",
    version = "0.1",
    type = "bin",
    compiler = "clang"
}

-- Excludes files from being compiled.
-- Useful when this file does not have a header

-- relative to the src directory
Exclude = {
    "test.c",
    "name.c"
}

-- relative to the src directory

-- default is main.c for bin and lib.c for lib projects
Entry = "lib.c"

-- external dependents
Dependencies = {
    -- { "dependency_name", 0.1 }
}
