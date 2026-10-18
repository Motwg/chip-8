use std::error::Error;

use wayland_client::{
    Connection, Dispatch, EventQueue, Proxy, QueueHandle,
    globals::{BindError, GlobalList, GlobalListContents, registry_queue_init},
    protocol::wl_registry,
};

pub struct State;
pub struct Ignore;

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(
        _state: &mut Self,
        _proxy: &wl_registry::WlRegistry,
        _event: wl_registry::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

#[macro_export]
macro_rules! wl_no_events {
    ( $( $t:ty ),* $(,)? ) => {
        $(
            impl ::wayland_client::Dispatch<$t,  $crate::wayland::registry::Ignore>
            for $crate::wayland::registry::State {
                fn event(
                    _state: &mut Self,
                    _proxy: &$t,
                    _event: <$t as ::wayland_client::Proxy>::Event,
                    _data: &$crate::wayland::registry::Ignore,
                    _conn: &::wayland_client::Connection,
                    _qhandle: &::wayland_client::QueueHandle<Self>,
                ) {}
            }
        )*
    };
}

pub struct Wayland<T> {
    _conn: Connection,
    globals: GlobalList,
    _queue: EventQueue<T>,
    qh: QueueHandle<T>,
}
impl<T> Wayland<T>
where
    T: Dispatch<wl_registry::WlRegistry, GlobalListContents> + 'static,
{
    pub fn connect() -> Result<Self, Box<dyn Error>> {
        let conn = Connection::connect_to_env()?;
        let (globals, queue) = registry_queue_init::<T>(&conn)?;
        let qh = queue.handle();
        Ok(Self {
            _conn: conn,
            globals,
            _queue: queue,
            qh,
        })
    }

    pub fn get<I, U>(&self, udata: U) -> Result<I, BindError>
    where
        I: Proxy + 'static,
        U: Send + Sync + 'static,
        T: Dispatch<I, U> + 'static,
    {
        self.globals.bind(&self.qh, 1..=1, udata)
    }
}
