use std::ptr;

use log::warn;
use qt5qml::core::{ConnectionTypeKind, QObject, QObjectRef, QString};
use qt5qml::QBox;
use qt5qml::{signal, slot};

use crate::player::qtgateway::LibrespotGateway;
use crate::player::LibrespotThread;

include!(concat!(env!("OUT_DIR"), "/qffi_Librespot.rs"));

pub struct LibrespotPrivate {
    qobject: *mut Librespot,
    thread: Option<LibrespotThread>,
    username: String,
}

impl LibrespotPrivate {
    pub fn new(qobject: *mut Librespot) -> Self {
        let gateway: QBox<LibrespotGateway> = LibrespotGateway::new(ptr::null_mut());
        QObject::connect(
            gateway.as_qobject(),
            signal!("playerEvent(const QString&)"),
            unsafe { &*qobject }.as_qobject(),
            slot!("onPlayerEvent(const QString&)"),
            ConnectionTypeKind::Queued,
        );

        Self {
            qobject,
            thread: Some(LibrespotThread::run(gateway)),
            username: "".to_string(),
        }
    }

    pub fn username(&self) -> QString {
        QString::from_utf8(&self.username)
    }

    pub fn set_username(&mut self, value: &QString) {
        self.username = value.to_string();
    }

    pub fn on_player_event(&mut self, event: &QString) {
        warn!("GOT event: {}", event)
    }
}

impl Drop for LibrespotPrivate {
    fn drop(&mut self) {
        if let Some(thread) = std::mem::replace(&mut self.thread, None) {
            thread.shutdown()
        }
    }
}
