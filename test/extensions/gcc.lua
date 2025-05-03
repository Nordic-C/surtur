-- extension:compiler_ext

---@param c_std string
---@param out_name string
---@param libraries string[]
---@param files string[]
---@param type string
---@param debug boolean
---@return string
local function generate_gcc_command(c_std, out_name, libraries, files, type, debug)
    -- Initialize the GCC command
    local gcc_command = "gcc"

    -- Add the C standard if provided
    if c_std then
        gcc_command = gcc_command .. " -std=" .. c_std
    end

    -- Add debugging flag if in debug mode
    if debug then
        gcc_command = gcc_command .. " -g"
    end

    -- Add the files to be compiled
    for _, file in ipairs(files) do
        gcc_command = gcc_command .. " " .. file
    end

    -- Add libraries to link with
    for _, lib in ipairs(libraries) do
        gcc_command = gcc_command .. " -l" .. lib
    end

    -- Set the output type: binary or library
    if type == "bin" then
        gcc_command = gcc_command .. " -o " .. out_name
    elseif type == "lib" then
        gcc_command = gcc_command .. " -o liboutput.so -fPIC -shared"
    end

    -- Return the final GCC command
    return gcc_command
end

-- Example usage:
local c_std = "gnu11"
local out_name = "main"
local libraries = { "m", "pthread" }
local files = { "main.c", "utils.c" }
local type = "bin" -- or "lib"
local debug = true

local command = generate_gcc_command(c_std, out_name, libraries, files, type, debug)
print(command)
