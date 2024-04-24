Control = {}
Control.__index = Control
Control.__tostring = function(self)
    local str = string.format(
        '{"thrust": %f,"elevator": %f,"aileron": %f,"rudder": %f}',
        self.thrust, self.elevator,
        self.aileron, self.rudder)
    return str
end
Control.__eq = function(self, other)
    return self.thrust == other.thrust and
        self.elevator == other.elevator and
        self.aileron == other.aileron and
        self.rudder == other.rudder
end

function Control.new(thrust, elevator, aileron, rudder)
    local obj = {}
    setmetatable(obj, Control)
    if type(thrust) == "table" then
        obj.thrust = thrust.thrust ~= nil and thrust.thrust or 0.0
        obj.elevator = thrust.elevator ~= nil and thrust.elevator or 0.0
        obj.aileron = thrust.aileron ~= nil and thrust.aileron or 0.0
        obj.rudder = thrust.rudder ~= nil and thrust.rudder or 0.0
    else
        if type(thrust) == "number" then
            obj.thrust = thrust ~= nil and thrust or 0.0
            obj.elevator = elevator ~= nil and elevator or 0.0
            obj.aileron = aileron ~= nil and aileron or 0.0
            obj.rudder = rudder ~= nil and rudder or 0.0
        else
            error("Invalid argument #1, expected number or table")
        end
    end
    return obj
end

function Control.default()
    return Control.new(0.0, 0.0, 0.0, 0.0)
end

module = Control
return module
