use std::error::Error;

use wayland_client::{
    Connection, Dispatch, DispatchError, EventQueue, Proxy, QueueHandle,
    backend::WaylandError,
    globals::{BindError, GlobalList, GlobalListContents, registry_queue_init},
    protocol::wl_registry,
};

pub struct Ignore;

macro_rules! wl_state {
    ( $state:ty ) => {
        impl
            ::wayland_client::Dispatch<
                ::wayland_client::protocol::wl_registry::WlRegistry,
                ::wayland_client::globals::GlobalListContents,
            > for $state
        {
            fn event(
                _state: &mut Self,
                _proxy: &::wayland_client::protocol::wl_registry::WlRegistry,
                _event: ::wayland_client::protocol::wl_registry::Event,
                _data: &::wayland_client::globals::GlobalListContents,
                _conn: &::wayland_client::Connection,
                _qhandle: &::wayland_client::QueueHandle<Self>,
            ) {
                println!("Registering state {}", ::std::any::type_name::<$state>());
            }
        }
    };
}

macro_rules! wl_no_events {
    ( $state:ty; $( $t:ty ),* $(,)? ) => {
        $(
            impl ::wayland_client::Dispatch<$t,  $crate::registry::Ignore>
            for $state {
                fn event(
                    _state: &mut Self,
                    _proxy: &$t,
                    _event: <$t as ::wayland_client::Proxy>::Event,
                    _data: &$crate::registry::Ignore,
                    _conn: &::wayland_client::Connection,
                    _qhandle: &::wayland_client::QueueHandle<Self>,
                ) {}
            }
        )*
    };

}

pub struct Wayland<T> {
    conn: Connection,
    globals: GlobalList,
    queue: EventQueue<T>,
    qh: QueueHandle<T>,
    state: T,
}
impl<T> Wayland<T>
where
    T: Dispatch<wl_registry::WlRegistry, GlobalListContents> + 'static,
{
    pub fn connect(state: T) -> Result<Self, Box<dyn Error>> {
        let conn = Connection::connect_to_env()?;
        let (globals, queue) = registry_queue_init::<T>(&conn)?;
        let qh = queue.handle();
        Ok(Self {
            conn,
            globals,
            queue,
            qh,
            state,
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

    pub fn qh(&self) -> &QueueHandle<T> {
        &self.qh
    }

    pub fn flush(&self) -> Result<(), WaylandError> {
        self.conn.flush()
    }

    pub fn roundtrip(&mut self) -> Result<usize, DispatchError> {
        self.queue.roundtrip(&mut self.state)
    }

    pub fn state(&self) -> &T {
        &self.state
    }
}
