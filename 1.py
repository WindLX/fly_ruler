import socket
import core_output_pb2
import pandas as pd
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

# %matplotlib widget

fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')

data_points = {'npos': [], 'epos': [], 'altitude': []}


def parse_protobuf_data(data):
    message = core_output_pb2.ViewMessage()

    message.ParseFromString(data)

    return message.output.state


def update_plot():
    ax.clear()
    ax.set_xlabel('North Position')
    ax.set_ylabel('East Position')
    ax.set_zlabel('Altitude')
    ax.plot3D(data_points['npos'], data_points['epos'],
              data_points['altitude'], marker='o', linestyle='-')
    plt.pause(0.1)


def start_server(host, port):
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    server_socket.bind((host, port))

    server_socket.listen(1)
    print(f"服务器正在监听 {host}:{port}...")

    client_socket, client_address = server_socket.accept()
    print(f"接收到来自 {client_address} 的连接")
    try:
        while True:
            data = client_socket.recv(1024)
            # print(f"接收到数据: {data}")

            message = parse_protobuf_data(data)
            # print(f"消息内容: {message}")

            data_points['npos'].append(message.npos)
            data_points['epos'].append(message.epos)
            data_points['altitude'].append(message.altitude)

            update_plot()

            if not data:
                break

    except KeyboardInterrupt:
        client_socket.close()
        return


if __name__ == "__main__":
    host = "127.0.0.1"
    port = 19999

    start_server(host, port)
