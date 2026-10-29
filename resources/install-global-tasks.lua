local shipped = [==[
@TASKS@
]==]

import("core.base.json")

local function tasks_path()
    if is_host("windows") then
        local appdata = os.getenv("APPDATA")
        if appdata and appdata ~= "" then
            return path.join(appdata, "Zed", "tasks.json")
        end
    end
    local home = os.getenv("HOME") or os.getenv("USERPROFILE") or "."
    return path.join(home, ".config", "zed", "tasks.json")
end

local shipped_tasks = json.decode(shipped)
local dest = tasks_path()
local raw = ""
if os.isfile(dest) then
    raw = io.readfile(dest) or ""
end

-- Always replace xmake tasks so a template change reaches people who already
-- have the old list. Other tasks in the file stay.
local foreign = {}
if raw ~= "" then
    local decoded = json.decode(raw)
    if type(decoded) ~= "table" then
        return
    end
    for _, task in ipairs(decoded) do
        local label = ""
        if type(task) == "table" and type(task.label) == "string" then
            label = task.label
        end
        if label:sub(1, 7) ~= "xmake: " then
            foreign[#foreign + 1] = task
        end
    end
end

os.mkdir(path.directory(dest))
if #foreign == 0 then
    io.writefile(dest, shipped)
    return
end
for _, task in ipairs(shipped_tasks) do
    foreign[#foreign + 1] = task
end
io.writefile(dest, json.encode(foreign))
