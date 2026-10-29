import("core.project.config")
import("core.project.project")
-- This script is passed as one argv to `xmake lua -c`. It must not start with "-".
-- project.chdir retargets os.projectdir() because Zed starts xmake outside the project.

local projectdir = os.getenv("XMAKE_DEBUG_PROJECT_DIR")
if projectdir and projectdir ~= "" then
    project.chdir(projectdir)
    os.cd(projectdir)
end
config.load()

local function existing_file(target)
    local file = target:targetfile()
    if not file or file == "" or not os.isfile(file) then
        return nil
    end
    return file
end

local function emit(target)
    local file = existing_file(target)
    if not file then
        return false
    end
    print("XMAKE_DEBUG_PROGRAM=" .. file)
    local rundir = target:rundir()
    if rundir and rundir ~= "" then
        print("XMAKE_DEBUG_RUNDIR=" .. rundir)
    end
    return true
end

local ordered = project.ordertargets()

local function find_enabled_binary(name)
    for _, t in ipairs(ordered) do
        if t:name() == name and t:is_binary() and t:is_enabled() then
            return t
        end
    end
end

local raw = os.getenv("XMAKE_DEBUG_TARGET") or ""
if raw ~= "" then
    -- One name, or several names separated by newlines (`xmake build lib app`).
    -- A named binary with no file is not replaced. A name that is not a binary
    -- is skipped so a later binary in the same build can be launched.
    local names = {}
    for name in raw:gmatch("[^\r\n]+") do
        names[#names + 1] = name
    end
    for _, name in ipairs(names) do
        local t = find_enabled_binary(name)
        if t then
            if not emit(t) then
                raise("binary for xmake target '%s' is not on disk", name)
            end
            return
        end
    end
    raise("xmake target '%s' is not an enabled binary", names[1] or raw)
end

local fallback
for _, t in ipairs(ordered) do
    if t:is_binary() and t:is_enabled() then
        if t:is_default() and existing_file(t) then
            emit(t)
            return
        elseif not fallback and existing_file(t) then
            -- A missing earlier binary must not hide a later one that exists.
            fallback = t
        end
    end
end
if not fallback or not emit(fallback) then
    raise("xmake did not produce an enabled binary")
end
