use crate::player::LibrespotThread;
use log::info;
use qt5qml::core::QString;

include!(concat!(env!("OUT_DIR"), "/qffi_Librespot.rs"));

pub struct LibrespotPrivate {
    qobject: *mut Librespot,
    thread: Option<LibrespotThread>,
    username: String,
}

impl LibrespotPrivate {
    pub fn new(qobject: *mut Librespot) -> Self {
        info!("NEW");
        Self {
            qobject,
            thread: Some(LibrespotThread::run()),
            username: "".to_string(),
        }
    }

    pub fn username(&self) -> QString {
        QString::from_utf8(&self.username)
    }

    pub fn set_username(&mut self, value: &QString) {
        self.username = value.to_string();
    }
}

impl Drop for LibrespotPrivate {
    fn drop(&mut self) {
        info!("DROP");
        if let Some(thread) = std::mem::replace(&mut self.thread, None) {
            thread.shutdown()
        }
    }
}
