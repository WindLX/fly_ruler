Algorithm = {}
Algorithm.__index = Algorithm

function Algorithm.new(init_state, init_control, raw_update)
    local self = {
        last_state = init_state,
        last_control = init_control,
        last_time = 0.0,
        raw_update = raw_update
    }
    setmetatable(self, Algorithm)
    return self
end

function Algorithm:update(time, state, other)
    local control = self.raw_update(self, time, state, other)
    self.last_state = state
    self.last_time = time
    self.last_control = control
    return control
end

module = Algorithm
return module
