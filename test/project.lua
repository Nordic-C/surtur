-- properties
Name = "test"
Props = {
    std = "c17",
    version = "0.1",
    type = "bin",
    compiler = "gcc"
}

-- Excludes files from being compiled.
-- Useful when this file does not have a header

-- relative to the src directory
Exclude = {
    "test.c"
}

-- relative to the src directory

-- default is main.c
Entry = "main.c"

-- external dependents
Dependencies = {
    -- { "dependency_name", 0.1 }
}
