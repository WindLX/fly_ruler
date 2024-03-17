use lazy_static::lazy_static;
use mlua::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

lazy_static! {
    static ref RT: tokio::runtime::Runtime = {
        std::thread::spawn(|| RT.block_on(futures::future::pending::<()>()));
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
    };
    static ref GUARD: tokio::runtime::EnterGuard<'static> = RT.enter();
}

#[derive(Clone)]
struct LuaTcpListener {
    listener: Arc<Mutex<TcpListener>>,
}

impl LuaTcpListener {
    async fn bind(addr: &str) -> LuaResult<Self> {
        let listener = TcpListener::bind(addr).await;
        match listener {
            Ok(listener) => Ok(Self {
                listener: Arc::new(Mutex::new(listener)),
            }),
            Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
        }
    }

    async fn accept(&self) -> LuaResult<(LuaTcpStream, String)> {
        let (stream, addr) = self.listener.lock().unwrap().accept().await?;
        Ok((
            LuaTcpStream {
                stream: Arc::new(Mutex::new(stream)),
            },
            addr.to_string(),
        ))
    }
}

impl LuaUserData for LuaTcpListener {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut(
            "accept",
            |_lua, this, ()| async move { this.accept().await },
        );
        methods.add_method("clone", |_lua, this, ()| Ok(this.clone()));
    }
}

#[derive(Clone)]
struct LuaTcpStream {
    stream: Arc<Mutex<TcpStream>>,
}

impl LuaTcpStream {
    async fn connect(addr: &str) -> LuaResult<Self> {
        let stream = TcpStream::connect(addr).await;
        match stream {
            Ok(stream) => Ok(Self {
                stream: Arc::new(Mutex::new(stream)),
            }),
            Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
        }
    }

    async fn write(&mut self, data: &[u8]) -> LuaResult<()> {
        let e = self.stream.lock().expect("aaa").write(data).await;
        if let Err(e) = e {
            dbg!(&e);
            return Err(mlua::Error::RuntimeError(e.to_string()));
        }
        let e = self.stream.lock().unwrap().flush().await;
        if let Err(e) = e {
            dbg!(&e);
            return Err(mlua::Error::RuntimeError(e.to_string()));
        }
        Ok(())
    }

    async fn read(&mut self) -> LuaResult<Vec<u8>> {
        let mut buffer = vec![0; 1024];
        let n = self.stream.lock().unwrap().read(&mut buffer).await?;
        Ok(buffer[..n].to_vec())
    }

    async fn read_until(&mut self, byte: u8) -> LuaResult<Vec<u8>> {
        let mut buffer = vec![0; 1024];
        let mut stream = self.stream.lock().unwrap();
        let mut n = 0;
        loop {
            let b = stream.read_u8().await?;
            if b == byte {
                break;
            }
            buffer[n] = b;
            n += 1;
        }
        Ok(buffer[..n].to_vec())
    }

    fn try_read(&self) -> LuaResult<Option<Vec<u8>>> {
        let mut buffer = vec![0; 1024];
        let n = self.stream.lock().unwrap().try_read(&mut buffer)?;
        if n > 0 {
            Ok(Some(buffer[..n].to_vec()))
        } else {
            Ok(None)
        }
    }

    async fn writable(&self) -> LuaResult<()> {
        self.stream
            .lock()
            .unwrap()
            .writable()
            .await
            .map_err(|e| mlua::Error::RuntimeError(e.to_string()))
    }

    async fn readable(&self) -> LuaResult<()> {
        self.stream
            .lock()
            .unwrap()
            .readable()
            .await
            .map_err(|e| mlua::Error::RuntimeError(e.to_string()))
    }
}

impl LuaUserData for LuaTcpStream {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method_mut("write", |_lua, this, data: mlua::String| async move {
            this.write(data.as_bytes()).await
        });
        methods.add_async_method_mut("read", |lua, this, ()| async move {
            let data = this.read().await?;
            lua.create_string(data)
        });
        methods.add_async_method_mut("read_until", |lua, this, byte: u8| async move {
            let data = this.read_until(byte).await?;
            lua.create_string(data)
        });
        methods.add_method("try_read", |lua, this, ()| {
            let data = this.try_read()?;
            match data {
                Some(data) => Ok(LuaValue::String(lua.create_string(data)?)),
                None => Ok(LuaValue::Nil),
            }
        });
        methods.add_async_method(
            "writable",
            |_lua, this, ()| async move { this.writable().await },
        );
        methods.add_async_method(
            "readable",
            |_lua, this, ()| async move { this.readable().await },
        );
        methods.add_method("clone", |_lua, this, ()| Ok(this.clone()));
    }
}

async fn bind<'lua>(
    _lua: &'lua Lua,
    addr: mlua::String<'lua>,
) -> LuaResult<Arc<Mutex<LuaTcpListener>>> {
    Ok(Arc::new(Mutex::new(
        LuaTcpListener::bind(addr.to_str().unwrap()).await?,
    )))
}

async fn connect<'lua>(
    _lua: &'lua Lua,
    addr: mlua::String<'lua>,
) -> LuaResult<Arc<Mutex<LuaTcpStream>>> {
    Ok(Arc::new(Mutex::new(
        LuaTcpStream::connect(addr.to_str().unwrap()).await?,
    )))
}

#[mlua::lua_module]
fn tcp(lua: &Lua) -> LuaResult<LuaTable> {
    let _guard = &*GUARD;
    let exports = lua.create_table()?;
    let tcp_stream = lua.create_table()?;
    tcp_stream.set("connect", lua.create_async_function(connect)?)?;

    let tcp_listener = lua.create_table()?;
    tcp_listener.set("bind", lua.create_async_function(bind)?)?;

    exports.set("tcp_stream", mlua::Value::Table(tcp_stream))?;
    exports.set("tcp_listener", mlua::Value::Table(tcp_listener))?;

    let pending = lua.create_async_function(|_, ()| async move {
        tokio::task::yield_now().await;
        Ok(())
    })?;

    exports.set("pending", pending)?;
    Ok(exports)
}
