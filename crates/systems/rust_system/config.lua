server_addr = "127.0.0.1:2350"

model_root_path = "./models"

core_init_cfg = {
    time_scale = 1.0, -- optional
    sample_time = 10 -- ms optional
}

model_install_args = { { "./models/f16_model/data" } }

plane_init_cfg = {
    deflection = { 0.0, 0.0, 0.0 }, -- ele(deg) ail(deg) rud(deg) | optional

    trim_target = {
        altitude = 1000, -- ft
        velocity = 500    -- ft/s
    },

    -- optional
    trim_init = {
        alpha = 8.49, -- deg
        -- thrust(lbs) ele(deg) ail(deg) rud(deg)
        control = {
            thrust = 1000.0,
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
