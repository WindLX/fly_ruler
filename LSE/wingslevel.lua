Algorithm = require('algorithm')
Control = require('control')
Integrator = require('integrator')

WingsLevel = {}
WingsLevel.__index = WingsLevel

local delta_theta_g = 0.0
local L_theta = 57.3 * 0.75
local T_e = -0.008
local L_theta_dot = -57.3 * 0.07
local integrator = Integrator.new()

local function raw_update(self, time, state)
    local delta_theta = state.theta - self.last_state.theta
    local delta_q = state.q - self.last_state.q
    -- local delta_alpha = state.alpha - self.last_state.alpha
    -- local delta_v = state.velocity - self.last_state.velocity

    local elevator = self.last_control.elevator
        + L_theta * (delta_theta - delta_theta_g)
        + L_theta_dot * delta_q
        + L_theta / T_e * integrator:update(time, (delta_theta - delta_theta_g))

    print(elevator)

    local thrust = self.last_control.thrust

    local control = Control.new(thrust, elevator, 0.0, 0.0)
    return control
end

function WingsLevel.new(init_state, init_control)
    local algorithm = Algorithm.new(init_state, init_control, raw_update)
    local self = {
        algorithm = algorithm,
    }
    setmetatable(self, WingsLevel)
    return self
end

function WingsLevel:update(time, state)
    return self.algorithm.update(self.algorithm, time, state)
end

module = WingsLevel
return module
