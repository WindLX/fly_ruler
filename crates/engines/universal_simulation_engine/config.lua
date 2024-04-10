system = {
    model_root_path = "./models",
    controller_buffer = 10
}

log = {
    -- target[span{field=value}]=level site:https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html
    filter = "tokio_util::codec::framed_impl=error,fly_ruler_plugin::plugin::ffi=warn,info",
    dir = "logs",
    file = "app.log"
}

server = {
    addr = "127.0.0.1:2350",
    tick_timeout = 1500, -- ms
    read_rate = 1,
}

core_init_cfg = {
    clock_mode = {
        Realtime = true,
        -- Fixed = {
        --     time_scale = 1.0, -- optional
        --     sample_time = 50 -- ms optional     
        -- }
    },
}

model_install_args = { { "./models/f16_model/data" } }

plane_init_cfg = {
    deflection = { 0.0, 0.0, 0.0 }, -- ele(deg) ail(deg) rud(deg) | optional

    trim_target = {
        altitude = 5000, -- ft
        velocity = 500,    -- ft/s
        npos = 0.0,
        epos = 0.0,
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
