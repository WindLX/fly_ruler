fly_ruler.init_logger({
    timestamp = nil,
    target = "Stderr"
});

local system = fly_ruler.create_system()
system:set_dir("../config", "../plugins/model")

system:init({
    time_scale = 1.0, -- optional
    sample_time = 100 -- ms optional
})

for i, v in ipairs(system.models) do
    fly_ruler.info(string.format("Id: %d, Plugin: %s, State: %s", i, v.info.name, v.state))
end

system:enable_model(1, { "../plugins/model/f16_model/data" })

local plane_cfg = {
    deflection = { 0.0, 0.0, 0.0 }, -- ele(deg) ail(deg) rud(deg) | optional

    trim_target = {
        altitude = 15000, -- ft
        velocity = 500    -- ft/s
    },

    -- optional
    trim_init = {
        alpha = 8.49, -- deg
        -- thrust(lbs) ele(deg) ail(deg) rud(deg)
        control = {
            thrust = 5000.0,
            elevator = -0.09,
            aileron = 0.01,
            rudder = -0.01
        }
    },

    flight_condition = "WingsLevel", -- "WingsLevel" | "Turning" | "PullUp" | "Roll" | optional

    -- -- optional
    optim_options = {
        max_fun_evals = 50000,
        max_iter = 10000,
        tol_fun = 1e-10,
        tol_x = 1e-10
    }
}

system:push_plane(1, plane_cfg)
system:push_plane(1, plane_cfg)

local init_control = {
    thrust = 5000.0,
    elevator = -0.09,
    aileron = 0.01,
    rudder = -0.01
}

local controller_1 = system:set_controller(1, init_control)
local controller_2 = system:set_controller(2, init_control)

local controller_co = coroutine.create(function ()
    local count = 0
    while true do
        count = count + 1
        if count == 10 then
            controller_2:send(fly_ruler.create_command("Exit"))
            break
        end
        coroutine.yield()
    end
end)

local viewer_1 = system:get_viewer(1)
local viewer_2 = system:get_viewer(2)

local viewer_co = coroutine.create(function()
    while true do
        local output = viewer_1:receive()
        if output ~= nil then
            print(output.data:to_table())
            print("Plane 1: " .. output.time)
        end
        local output = viewer_2:receive()
        if output ~= nil then
            print("Plane 2: " .. output.time)
        end
        coroutine.yield()
    end
end)

system:start()
local count = 0
while count ~= 15 do
    count  = count + 1
    system:step(false)
    coroutine.resume(viewer_co)
    coroutine.resume(controller_co)
end

system:disable_model(1)
system:stop()
