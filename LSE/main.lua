package.cpath = package.cpath ..
    ';.\\modules\\?.dll;.\\modules\\?.so';

-- dll/so
local LSE = require("light_simulation_engine")

--  lua
local config = require("config")
local Control = require("control")

LSE.logger.init(config.logger_init_cfg)

Ltrace = LSE.logger.trace
Ldebug = LSE.logger.debug
Linfo = LSE.logger.info
Lwarn = LSE.logger.warn
Lerror = LSE.logger.error

local system = LSE.system.new()
system:set_dir(config.model_dir)
system:init(config.system_init_cfg)

local keys = {}

for k, v in pairs(system.models) do
    system:enable_model(k, config.F16_install_args)
    Linfo(string.format("Id: %s, Model: %s, State: %s", tostring(k), v.info.name, v.state))
    keys[#keys + 1] = k
end

local init_control = Control:new(config.plane_init_cfg.trim_init.control)

local id, viewer = table.unpack(system:push_plane(keys[1], nil, config.plane_init_cfg))
local controller = system:set_controller(id, 10)

local time = 0
local exit_flag = false

local system_thread = coroutine.create(function()
    local sys = system:clone()
    while not exit_flag do
        xpcall(function()
            local r = sys:step()
            if type(r) == "string" then
                print(r)
            end
        end, print)
        coroutine.yield()
    end
end)

local viewer_thread = coroutine.create(function()
    while not exit_flag do
        if viewer:has_changed() then
            local output = viewer:get_and_update()
            time = output.time
            print(time)
            if time >= 10 then
                exit_flag = true
            end
        end
        coroutine.yield()
    end
end)

local controller_thread = coroutine.create(function()
    while not exit_flag do
        local last_control = init_control
        xpcall(function() controller:send(last_control) end, print)
        coroutine.yield()
    end
end)

while not exit_flag do
    coroutine.resume(system_thread, system)
    coroutine.resume(viewer_thread)
    coroutine.resume(controller_thread)
end

system:disable_model(keys[1])
system:stop()
