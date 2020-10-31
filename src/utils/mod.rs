use crate::player::Options;
use qt5qml::core::QString;

pub mod xdg;

pub struct UnsafeSend<T>(T);

impl<T> UnsafeSend<T> {
    pub fn new(t: T) -> Self {
        Self(t)
    }

    pub unsafe fn unwrap(self) -> T {
        self.0
    }
}

unsafe impl<T> Send for UnsafeSend<T> {}

pub fn to_qstring(s: Option<&String>) -> QString {
    if let Some(ref s) = s {
        QString::from_utf8(s)
    } else {
        QString::new()
    }
}

pub fn from_qstring(s: &QString) -> Option<String> {
    // TODO: !s.is_null()
    if s.len() != 0 {
        Some(s.to_string())
    } else {
        None
    }
}
