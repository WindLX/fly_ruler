package.cpath = package.cpath ..
    ';.\\modules\\?.dll;.\\modules\\?.so';

local json_viewer = require("json_viewer")
local tcp = require("tcp")

local config = require("config")
local Command = require("command")

POLL_PENDING = coroutine.wrap(tcp.pending)()

local client_thread = coroutine.create(function(addr)
    local server
    xpcall(function()
        server = tcp.tcp_stream.connect(addr)
    end, print)
    if server ~= POLL_PENDING then
        coroutine.yield(server)
    end
end)

::start::
local _, server = coroutine.resume(client_thread, config.ServerAddr)
if server ~= POLL_PENDING then
    print("Connect to %s", config.ServerAddr)
    local init_cmd = Command:new_control(config.PlaneInitCfg.trim_init.control)

    local read_thread = coroutine.create(function()
        local disconnect_detected = false
        local server = server:clone()
        while true do
            local function f()
                server:readable()
                local bytes = server:read_until(string.byte("\n"))
                if bytes then
                    print("Receive: " .. bytes)
                    coroutine.yield()
                else
                    print("Disconnect")
                    disconnect_detected = true
                end
            end
            xpcall(f, print)
            if disconnect_detected then break end
        end
    end)

    local write_thread = coroutine.create(function()
        local disconnect_detected = false
        local server = server:clone()
        while true do
            local function f()
                server:writable()
                server:write(tostring(init_cmd) .. "\n")
                -- print("Send successfully")
            end
            xpcall(f, function(e)
                print(e)
                print("Disconnect")
                disconnect_detected = true
            end)
            if disconnect_detected then break end
            coroutine.yield()
        end
    end)

    while true do
        -- coroutine.resume(write_thread)
        coroutine.resume(read_thread)
        -- print(coroutine.status(read_thread))
    end
else
    goto start
end
