local root = os.getenv("XMAKE_ZED_ROOT")
-- Patterns start with a slash so they match the absolute path. That is what
-- outranks the Lua extension's ".lua" suffix. A folder with no xmake.lua is
-- left alone.
if not root or root == "" then
    return
end
if not os.isfile(path.join(root, "xmake.lua")) then
    return
end

import("core.base.json")

local dest_dir = path.join(root, ".zed")
local dest = path.join(dest_dir, "settings.json")
local patterns = {"/**/*.lua", "?:/**/*.lua"}
local settings = {}

if os.isfile(dest) then
    local raw = io.readfile(dest) or ""
    if raw:find("//", 1, true) or raw:find("/*", 1, true) then
        return
    end
    local decoded = json.decode(raw)
    if type(decoded) ~= "table" then
        return
    end
    settings = decoded
end

if type(settings.file_types) ~= "table" then
    settings.file_types = {}
end

local list = settings.file_types.xmake
if type(list) ~= "table" then
    list = {}
end

local have = {}
for _, item in ipairs(list) do
    if type(item) == "string" then
        have[item] = true
    end
end

local changed = false
for _, pattern in ipairs(patterns) do
    if not have[pattern] then
        list[#list + 1] = pattern
        changed = true
    end
end
if not changed then
    return
end

settings.file_types.xmake = list
os.mkdir(dest_dir)
io.writefile(dest, json.encode(settings))
