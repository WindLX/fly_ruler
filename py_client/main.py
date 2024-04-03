import flyruler_py_client as fr
import asyncio


async def main():
    client = await fr.PyClient.new("127.0.0.1", 2350)
    model_infos = await client.get_model_infos()
    for model_info in model_infos:
        print(f"Id {model_info.id}, Name {model_info.info.name}")
    f16_id = fr.UuidWrapper.parse_str(model_infos[0].id)
    plane_id = await client.push_plane((f16_id, None))
    print(f"New F16: {plane_id}")
    client.tick(1000)
    control = fr.ControlWrapper(5000, -0.09, 0.01, -0.01)
    await asyncio.sleep(0.5)
    count = 0
    while count < 100:
        await client.send_control((plane_id, control))
        msg = await client.output()
        id, time, output = msg.id, msg.time, msg.output
        print(f"{id}, {time}, npos:{output.state.npos}")
        await asyncio.sleep(0.01)
        count += 1
    await client.stop()


main_coro = main()

loop = asyncio.get_event_loop()
task = loop.create_task(main_coro)

loop.run_until_complete(task)
loop.close()
