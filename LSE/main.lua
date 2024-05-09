local LSE = require("lse")

--  lua
local config = require("config")
local CSVWriter = require("csv")
local WingsLevel = require("wingslevel")
local Algorithm = require("algorithm")

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

local id, viewer, controller, handler, ctk, trim_output = table.unpack(system:push_plane(keys[1], 10,
    config.plane_init_cfg))

END_TIME = 15
local exit_flag = false

local system_thread = coroutine.create(function()
    xpcall(
        function()
            coroutine.yield(handler:wait())
        end,
        function(e)
            print(e)
            coroutine.yield(nil)
        end)
end)

local csv_writer = CSVWriter.new("data.csv")

local init_state = trim_output.state
local init_control = trim_output.control
local wings_level = WingsLevel.new(init_state, init_control)

local main_thread = coroutine.create(function()
    local time = 0
    while not exit_flag do
        xpcall(function() controller:send(init_control) end, print)
        LSE.sleep(1)

        if viewer:has_changed() then
            local output = viewer:get_and_update()
            time = output.time
            local state = output.data.state

            local control = wings_level:update(time, state)
            init_control = control

            csv_writer:write(output)

            if time >= END_TIME then
                exit_flag = true
                ctk:cancel()
                csv_writer:close()
                wings_level.csv_writer.close(wings_level.csv_writer)
                Linfo("Exit")
            end
        end
        coroutine.yield()
    end
end)

while not exit_flag do
    coroutine.resume(main_thread)
end

while true do
    local status, result = coroutine.resume(system_thread)
    if result == nil then
        break
    end
end

system:disable_model(keys[1])
system:stop()

os.execute(string.format("cp %s %s", "data.csv", "../crates/modules/chart/chart/public/data.csv"))
