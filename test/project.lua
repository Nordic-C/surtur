-- properties
Name = "test"
Props = {
    std = "c17",
    version = "0.1",
    type = "bin",
    compiler = "gcc" --"x86_64-w64-mingw32-gcc",
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

WinLibraries = {
    "winraylib",
    "opengl32",
    "gdi32",
    "winmm",
}

Params = {
    -- "--deez"
}
