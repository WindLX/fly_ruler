local fly_ruler = require("lua_system")

module = {
    ErrorHandler = function(err)
        fly_ruler.logger.error(err)
    end,

    Ltrace = fly_ruler.logger.trace,
    Ldebug = fly_ruler.logger.debug,
    Linfo = fly_ruler.logger.info,
    Lwarn = fly_ruler.logger.warn,
}

return module
