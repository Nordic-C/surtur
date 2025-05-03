-- properties
Name = "test"
Props = {
    std = "c17",
    version = "0.1",
    type = "bin",
    compiler = "clang",
}

-- external dependenciess
Dependencies = {
    -- { "https://github.com/Surtur-Team/surtests", 0.1 }
}

Parameters = {
    "DEBUG",
    "RELEASE"
}

Libraries = {
    "raylib",
    "GL",
    "m",
    "pthread",
    "dl",
    "rt",
    "X11",
}
