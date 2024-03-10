local system = create_system()
system:set_dir("../config", "../plugins/model")

system:init({
    time_scale = 1.0, -- optional
    sampel_time = 100 -- ms optional
})

for i, v in ipairs(system.models) do
    print(string.format("Id: %d, Plugin: %s, State: %s", i, v.info.name, v.state))
end

system:enable_model(1, { "../plugins/model/f16_model/data" })
print(system:get_model_state(1))

system:push_plane(1, {
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
})

local init_control = {
    thrust = 5000.0,
    elevator = -0.09,
    aileron = 0.01,
    rudder = -0.01
}

local controller = system:get_controller(1, init_control)
system:run(false)

system:disable_model(1)
system:stop()
