package.cpath = package.cpath ..
    ';.\\modules\\?.dll;.\\modules\\?.so';

-- dll/so
local fly_ruler = require("lua_system")
local protobuf_viewer = require("protobuf_viewer")
local json_viewer = require("json_viewer")
local tcp = require("tcp")

--  lua
local config = require("config")
local Command = require("command")

POLL_PENDING = coroutine.wrap(tcp.pending)()
fly_ruler.logger.init(config.LoggerInitCfg)

Ltrace = fly_ruler.logger.trace
Ldebug = fly_ruler.logger.debug
Linfo = fly_ruler.logger.info
Lwarn = fly_ruler.logger.warn
Lerror = fly_ruler.logger.error

local server_thread = coroutine.create(function(addr)
    xpcall(function()
        coroutine.yield(tcp.tcp_listener.bind(addr))
    end, print)
end)
local _, listener = coroutine.resume(server_thread, config.ServerAddr)
Linfo(string.format("Server running at %s", config.ServerAddr))

local listen_thread = coroutine.create(function()
    while true do
        local client, client_addr = listener:accept()
        Linfo(string.format("Client connecting: %s", client_addr))
        if client_addr ~= POLL_PENDING then
            coroutine.yield(client, client_addr)
        end
    end
end)

local system = fly_ruler.system.new()
system:set_dir(config.ModelDir)
system:init(config.SystemInitCfg)

local keys = {}

for k, v in pairs(system.models) do
    system:enable_model(k, config.F16InstallArgs)
    Linfo(string.format("Id: %s, Model: %s, State: %s", tostring(k), v.info.name, v.state))
    keys[#keys + 1] = k
end


local init_cmd = Command:new_control(config.PlaneInitCfg.trim_init.control)

local client_set = {}
local viewer_set = {}
local controller_set = {}
local id_set = fly_ruler.uuid_set.new()

local system_thread = coroutine.create(function()
    local sys = system:clone()
    while true do
        xpcall(function() sys:step() end, print)
        coroutine.yield()
    end
end)

while true do
    local _, client, client_addr = coroutine.resume(listen_thread)

    if client ~= POLL_PENDING then
        -- system_thread = nil
        -- collectgarbage()

        local id, viewer = table.unpack(system:push_plane(keys[1], config.PlaneInitCfg))
        local controller = system:set_controller(id, 10)
        client_set[tostring(id)] = client
        id_set:add(id)
        Linfo(string.format("Client %s connect Plane %s", client_addr, tostring(id)))

        local viewer_thread = coroutine.create(function(encoder)
            local disconnect_detected = false
            local client_addr = client_addr
            while true do
                if viewer:has_changed() then
                    local output = viewer:get_and_update()
                    local msg = {
                        time = output.time,
                        view_msg = { { tostring(id), output.data } }
                    }
                    local chars = encoder.encode(msg)
                    for index, id in ipairs(id_set:to_table()) do
                        local function f()
                            local id = tostring(id)
                            if client_set[id] then
                                -- client_set[id]:writable()
                                client_set[id]:write(chars)
                            end
                        end
                        xpcall(f, function(e)
                            print(e)
                            Lwarn(string.format("Client %s at %s disconnect", tostring(id), client_addr))
                            disconnect_detected = true
                        end)
                    end
                end
                if disconnect_detected then
                    break
                end
                coroutine.yield()
            end
        end)

        local controller_thread = coroutine.create(function(encoder)
            local client = client:clone()
            local id = id
            local client_addr = client_addr
            local disconnect_detected = false
            while true do
                local last_command = init_cmd
                local function f()
                    client:readable()
                    local bytes = client:read_until(string.byte('\n'))
                    if bytes then
                        last_command = encoder.decode_cmd(bytes)
                    end
                    controller:send(last_command)
                end
                xpcall(f, function(e)
                    print(e)
                    Lwarn(string.format("Client %s at %s disconnect", tostring(id), client_addr))
                    disconnect_detected = true
                end)
                if disconnect_detected then
                    break
                end
                coroutine.yield()
            end
        end)

        viewer_set[tostring(id)] = viewer_thread
        controller_set[tostring(id)] = controller_thread
        -- system_thread = coroutine.create(step)
        Ldebug("Create a new client handler")
    end

    if #id_set ~= 0 then
        coroutine.resume(system_thread, system)
    end

    for index, id in ipairs(id_set:to_table()) do
        local function exit_handler(id)
            -- system_thread = nil
            -- collectgarbage()

            local idx = tostring(id)
            viewer_set[idx] = nil
            controller_set[idx] = nil
            client_set[idx] = nil
            id_set:remove(id)
            system:remove_plane(id)
            -- system_thread = coroutine.create(step)

            Ldebug("Remove a client handler")
        end

        local idx = tostring(id)
        if viewer_set[idx] ~= nil then
            if coroutine.status(viewer_set[idx]) == "dead" then
                exit_handler(id)
            else
                coroutine.resume(viewer_set[idx], json_viewer)
            end
        end
        if controller_set[idx] ~= nil then
            if coroutine.status(controller_set[idx]) == "dead" then
                exit_handler(id)
            else
                coroutine.resume(controller_set[idx], json_viewer)
            end
        end
    end
end

system:disable_model(1)
system:stop()
