Command = {}
Command.__index = Command
Command.__tostring = function(self)
    if self.control ~= nil then
        local cmd = string.format(
            '{"thrust": %f,"elevator": %f,"aileron": %f,"rudder": %f}',
            self.control.thrust, self.control.elevator,
            self.control.aileron, self.control.rudder)
        return cmd
    else
        return "{ 'exit' }"
    end
end

function Command:new_control(control)
    local obj = {}
    setmetatable(obj, Command)
    obj.control = control
    return obj
end

function Command:new_extra(extra)
    local obj = {}
    setmetatable(obj, Command)
    obj.extra = extra
    return obj
end

function Command:new_exit()
    local obj = {}
    setmetatable(obj, Command)
    obj.exit = true
    return obj
end

function Command:default()
    return Command:new_control({
        thrust = 0.0,
        elevator = 0.0,
        aileron = 0.0,
        rudder = 0.0
    })
end

module = Command
return module
