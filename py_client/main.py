import flyruler_py_client as fr
import asyncio

fr.register_logger("tokio_util::codec::framed_impl=error,info", None, None)

plane_init_cfg = fr.PlaneInitCfgWrapper(
    deflection = [ 0.0, 0.0, 0.0 ],

    trim_target = fr.TrimTargetWrapper(
        altitude = 10000,
        velocity = 500,
        npos = 10000.0,
        epos = 0.0
    ),

    trim_init = fr.TrimInitWrapper(
        alpha = 8.49,
        control = fr.ControlWrapper(
            thrust = 5000.0,
            elevator = -0.09,
            aileron = 0.01,
            rudder = -0.01
        )
    ),

    flight_condition = fr.FlightConditionWrapper.wings_level(),

    optim_options = fr.NelderMeadOptionsWrapper(
        max_fun_evals = 50000,
        max_iter = 10000,
        tol_fun = 1e-10,
        tol_x = 1e-10
    )
)

async def main():
    client = await fr.PyClient.new("127.0.0.1", 2350)
    client.tick(1000)
    model_infos = await client.get_model_infos()
    for model_info in model_infos:
        print(f"Id {model_info.id}, Name {model_info.info.name}")
    f16_id = fr.UuidWrapper.parse_str(model_infos[0].id)
    plane_id = await client.push_plane((f16_id, plane_init_cfg))
    print(f"New F16: {plane_id}")
    
    control = fr.ControlWrapper(5000, -0.09, 0.01, -0.01)
    await asyncio.sleep(0.5)
    count = 0
    await client.send_control((plane_id, control))
    while count < 1e10:
        msg = await client.output()
        id, time, output = msg.id, msg.time, msg.output
        if id == plane_id:
            await client.send_control((plane_id, control))
            print(f"{id}, {time}, npos:{output.state.npos}")
            await asyncio.sleep(0.05)
        count += 1
    await client.stop()


main_coro = main()

loop = asyncio.get_event_loop()
task = loop.create_task(main_coro)

loop.run_until_complete(task)
loop.close()
