use std::ptr;

use log::info;
use qt5qml::core::{ConnectionTypeKind, QByteArray, QObject, QObjectRef, QString};
use qt5qml::QBox;
use qt5qml::{cstr, signal, slot};

use crate::player::qtgateway::{deserialize_event, LibrespotGateway};
use crate::player::{LibrespotThread, Options};

include!(concat!(env!("OUT_DIR"), "/qffi_Librespot.rs"));

enum PlayerState {
    Disconnected,
    Paused,
    Playing,
    Buffering,
}

pub struct LibrespotPrivate {
    qobject: *mut Librespot,
    thread: Option<LibrespotThread>,
    username: String,
    password: String,

    last_error: Option<String>,
    state: PlayerState,
    track: Option<String>,
}

pub fn register_librespot() {
    Librespot::register_type(cstr!("Sailify"), 0, 1, cstr!("Librespot"));
}

impl LibrespotPrivate {
    pub fn new(qobject: *mut Librespot) -> Self {
        Self {
            qobject,
            thread: None,
            username: "".to_string(),
            password: "".to_string(),

            last_error: None,
            state: PlayerState::Paused,
            track: None,
        }
    }

    pub fn username(&self) -> QString {
        QString::from_utf8(&self.username)
    }

    pub fn set_username(&mut self, value: &QString) {
        self.username = value.to_string();
    }

    // #[property(write = set_password, notify = password_changed)]
    pub fn password(&self) -> QString {
        QString::from_utf8(&self.password)
    }

    pub fn set_password(&mut self, value: &QString) {
        self.password = value.to_string();
    }

    // #[slot]
    pub fn on_player_event(&mut self, event: &QByteArray) {
        let evt = deserialize_event(event.as_slice());
        info!("GOT event: {:?}", evt);
    }

    // #[slot]
    pub fn start(&mut self) {
        if self.is_active() {
            return;
        }

        let mut opts = Options::new();
        opts.username = self.username.clone();
        opts.password = self.password.clone();

        let mut gateway: QBox<LibrespotGateway> = LibrespotGateway::new(ptr::null_mut());
        QObject::connect(
            gateway.as_qobject(),
            signal!("playerEvent(const QByteArray&)"),
            unsafe { &mut *self.qobject }.as_qobject(),
            slot!("_onPlayerEvent(const QByteArray&)"),
            ConnectionTypeKind::Queued,
        );
        gateway.move_to_thread(None);
        self.thread = Some(LibrespotThread::run(gateway, opts));

        unsafe { &mut *self.qobject }.activeChanged(true);
    }

    // #[slot]
    pub fn stop(&mut self) {
        if !self.is_active() {
            return;
        }

        self.shutdown();

        unsafe { &mut *self.qobject }.activeChanged(false);
    }

    fn shutdown(&mut self) {
        if let Some(thread) = std::mem::replace(&mut self.thread, None) {
            thread.shutdown()
        }
    }

    pub fn is_active(&self) -> bool {
        self.thread.is_some()
    }
}

impl Drop for LibrespotPrivate {
    fn drop(&mut self) {
        self.shutdown()
    }
}
