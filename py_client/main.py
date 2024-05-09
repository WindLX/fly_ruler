import flyruler_py_client as fr
import asyncio
from multiprocessing import  Process
import timeit

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

async def main_task(n):
    response = []
    client = await fr.PyClient.new("127.0.0.1", 2350)
    client.tick(500)
    model_infos = await client.get_model_infos()
    for model_info in model_infos:
        print(f"Id {model_info.id}, Name {model_info.info.name}")
    f16_id = fr.UuidWrapper.parse_str(model_infos[0].id)
    
    control = fr.ControlWrapper(5000, -0.09, 0.01, -0.01)
    plane_ids = []
    
    for i in range(n):
        plane_id = await client.push_plane((f16_id, plane_init_cfg))
        print(f"New F-16 {i}: {plane_id}")
        plane_ids.append(plane_id)
        await client.send_control((plane_id, control))
    
    count = 0
    while count < 200:
        ccount = 0
        # for i in range(len(plane_ids)):
        while True:
            if ccount == len(plane_ids):
                break
            start_time = timeit.default_timer()
            msg = await client.output()
            execution_time = timeit.default_timer() - start_time
            print("Response time", execution_time, "s")
            response.append(execution_time)
            id, t, output = msg.id, msg.time, msg.output
            if id in plane_ids:
                ccount += 1
                await client.send_control((id, control))
                # print(f"{i} {id}, {time}, npos:{output.state.npos}")
        await asyncio.sleep(0.05)
        count += 1
        print(count)
    print("Average response time:", sum(response[1:]) / len(response[1:]) * 1000)
    await client.stop()


def main(n):
    main_coro = main_task(n)

    loop = asyncio.get_event_loop()
    task = loop.create_task(main_coro)

    loop.run_until_complete(task)
    loop.close()


if __name__ == '__main__':
    process_list = []
    for i in range(1):
        p = Process(target=main, args=(1,))
        p.start()
        process_list.append(p)

    for (i, p) in enumerate(process_list):
        p.join()
        print(f'Process {i} joined')

    print('结束测试')
    a = input()
    # main()