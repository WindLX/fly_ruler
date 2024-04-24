Integrator = {}
Integrator.__index = Integrator

function Integrator.new()
    local self = {
        init = 0.0,
        last_time = 0.0,
        last_value = 0.0,
        past = 0.0,
    }
    setmetatable(self, Integrator)
    return self
end

function Integrator:update(time, value)
    local dt = time - self.last_time
    local dv = value - self.last_value
    self.past = self.past + dt * (self.init + 0.5 * dv)
    self.last_time = time
    self.last_value = value
    return self.past
end

function Integrator:reset(time, value)
    self.init = value
    self.last_time = time
    self.last_value = value
    self.past = 0.0
end

function Integrator:get_value()
    return self.past
end

module = Integrator
return module
