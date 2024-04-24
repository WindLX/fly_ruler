package.cpath = package.cpath ..
    ';.\\modules\\?.dll;./modules/?.so';

-- dll/so
local plantform = "linux"

LSE = nil

if plantform == "windows" then
    package.cpath = package.cpath ..
        ';.\\modules\\?.dll'
    LSE = require("light_simulation_engine")
else
    package.cpath = package.cpath ..
        ';./modules/?.so'
    LSE = require("liblight_simulation_engine")
end

module = LSE
return module
