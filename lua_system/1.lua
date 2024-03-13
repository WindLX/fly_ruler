local tcp = require("tcp")

local listener = tcp.tcp_listener.bind("192.168.1.108:19991")
print("Server start")

local client, addr = listener:accept()
print("Client: " .. addr)

while true do
    local data = client:read()
    if #data ~= 0 then
        print(data)
    end
end
