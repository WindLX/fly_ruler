#include <stdio.h>
#include "test_utils.h"

const char *frutils_info_level_to_string(LogLevel level)
{
    switch (level)
    {
    case TRACE:
        return "TRACE";
    case DEBUG:
        return "DEBUG";
    case INFO:
        return "INFO";
    case WARN:
        return "WARN";
    case ERROR:
        return "ERROR";
    case FATAL:
        return "FATAL";
    default:
        return "INFO";
    }
}

void test_frutils_log(const char *msg, LogLevel level)
{
    printf("[%s] %s\n", frutils_info_level_to_string(level), msg);
}

int frsys_init()
{
    test_frutils_log("f16 model test start", INFO);
    frutils_register_logger(test_frutils_log);
    return 0;
}

int frsys_stop()
{
    test_frutils_log("f16 model test end", INFO);
    return 0;
}

int frmodel_install()
{
    int r = 0;
    r = frmodel_install_hook(1, "./data");
    return r;
}

int frmodel_uninstall()
{
    int r = 0;
    r = frmodel_uninstall_hook(0);
    return r;
}

int fr_main(FrsysStep frsys_step)
{
    int r = 0;
    r = frsys_init();
    if (r >= 0)
    {
        r = frmodel_install();
        if (r < 0)
        {
            r = frsys_stop();
            return r;
        }
        for (int i = 0; i < 1; i++)
        {
            r = frsys_step();
            if (r < 0)
            {
                r = frsys_stop();
                return r;
            }
        }
        r = frmodel_uninstall();
    }
    r = frsys_stop();
    return r;
}