local tcp = require("tcp")
local addr = "127.0.0.1:2345"

local pending = coroutine.wrap(tcp.pending)()
local thread = coroutine.create(function(addr)
    return tcp.tcp_listener.bind(addr)
end)

local status, listener = coroutine.resume(thread, addr)
print(listener)
print("Server: " .. addr)

local co = coroutine.create(function(listener)
    while true do
        local client, addr = listener:accept()
        print("Client: " .. addr)
    end
end)

while true do
   coroutine.resume(co, listener)
--    print(coroutine.status(co))
end

-- while true do
--     local data = client:read()
--     if #data ~= 0 then
--         print(data)
--     end
-- end
