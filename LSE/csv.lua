local LSE = require("lse")

CSVWriter = {}
CSVWriter.__index = CSVWriter

local function output_to_csv(output)
    local state, control, state_extend = output.data.state, output.data.control, output.data.state_extend
    local v = {
        output.time,
        state.npos,
        state.epos,
        state.altitude,
        LSE.to_degrees(state.phi),
        LSE.to_degrees(state.theta),
        LSE.to_degrees(state.psi),
        state.velocity,
        LSE.to_degrees(state.alpha),
        LSE.to_degrees(state.beta),
        LSE.to_degrees(state.p),
        LSE.to_degrees(state.q),
        LSE.to_degrees(state.r),
        control.thrust,
        control.elevator,
        control.aileron,
        control.rudder,
        state_extend.nx,
        state_extend.ny,
        state_extend.nz,
        state_extend.mach,
        state_extend.qbar,
        state_extend.ps
    }
    return v
end

function CSVWriter.new(filename)
    local self = setmetatable({}, CSVWriter)
    self.filename = filename
    self.file = io.open(filename, "w")
    self.header = {
        "time(s)",
        "npos(ft)", "epos(ft)", "altitude(ft)",
        "phi(degree)", "theta(degree)", "psi(degree)",
        "velocity(ft/s)",
        "alpha(degree)", "beta(degree)",
        "p(degree/s)", "q(degree/s)", "r(degree/s)",
        "thrust(lbs)", "elevator(degree)", "aileron(degree)", "rudder(degree)",
        "nx(g)", "ny(g)", "nz(g)",
        "mach", "qbar(lb/ft ft)", "ps(lb/ft ft)",
    }
    self.file:write(table.concat(self.header, ",") .. "\n")
    return self
end

function CSVWriter:write(data)
    self.file:write(table.concat(output_to_csv(data), ",") .. "\n")
end

function CSVWriter:close()
    self.file:close()
end

module = CSVWriter
return module
