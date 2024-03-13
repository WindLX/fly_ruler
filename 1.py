import socket

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(("192.168.1.108",19991))

# while True:
#     r = s.recv(1024)
#     print(r)
s.sendall("hello lua".encode())