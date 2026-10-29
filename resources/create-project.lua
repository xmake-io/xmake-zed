local lang = os.getenv("XMAKE_CREATE_LANG")
local template = os.getenv("XMAKE_CREATE_TEMPLATE")
if not lang or lang == "" or not template or template == "" then
    raise("xmake create task is missing a language or template")
end
local name = "app"
if template:find("static", 1, true) or template:find("shared", 1, true) or template:find("module", 1, true) or template:find("framework", 1, true) or template:find("bundle", 1, true) then
    name = "lib"
end
if not os.isfile("xmake.lua") then
    os.execv("xmake", {"create", "-P", ".", "-f", "-l", lang, "-t", template, name})
    return
end
local n = 2
local dir = "project2"
while os.isdir(dir) or os.isfile(dir) do
    n = n + 1
    dir = "project" .. tostring(n)
end
os.execv("xmake", {"create", "-l", lang, "-t", template, dir})
local lua = io.readfile("xmake.lua") or ""
local line = "includes(" .. string.char(34) .. dir .. string.char(34) .. ")"
if not lua:find(line, 1, true) then
    io.writefile("xmake.lua", line .. string.char(10) .. lua)
end
print("created ./" .. dir .. " and added it to xmake.lua")
