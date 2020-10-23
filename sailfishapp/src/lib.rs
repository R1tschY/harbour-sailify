#[macro_use]
extern crate cpp;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::pin::Pin;
use qt5qml::core::{QApplicationFactory, QObjectRef, QUrl, QString};
use qt5qml::gui::QGuiApplication;
use cpp::cpp;
use qt5qml::QBox;

mod quickview;

cpp! {{
    #include <sailfishapp.h>
    #include <QGuiApplication>

    #include <QQuickView>
}}


// TODO
#[repr(C)]
pub struct QQuickView {
    _private: [u8; 0],
}

impl qt5qml::core::QObjectRef for QQuickView {
    fn as_qobject_mut(&mut self) -> &mut qt5qml::core::QObject {
        unsafe { &mut *(self as *mut _ as *mut qt5qml::core::QObject) }
    }

    fn as_qobject(&self) -> &qt5qml::core::QObject {
        unsafe { &*(self as *const _ as *const qt5qml::core::QObject) }
    }
}

impl QQuickView {

    pub fn set_source(&mut self, url: &QUrl) {
        cpp!(unsafe [self as "QQuickView*", url as "const QUrl*"] {
            self->setSource(*url);
        })
    }

    pub fn show_full_screen(self: &mut QQuickView) {
        cpp!(unsafe [self as "QQuickView*"] {
            self->showFullScreen();
        })
    }
}

/// Sailfish application using booster
pub struct SailfishApp;

impl SailfishApp {
    pub fn create_view() -> QBox<QQuickView> {
        unsafe {
            let view = cpp!([] -> *mut QQuickView as "QQuickView*" {
                return SailfishApp::createView();
            });
            QBox::from_raw(view)
        }
    }

    pub fn path_to(value: &QString) -> QUrl {
        cpp!(unsafe [value as "const QString*"] -> QUrl as "QUrl" {
            return SailfishApp::pathTo(*value);
        })
    }

    pub fn path_to_main_qml() -> QUrl {
        cpp!(unsafe [] -> QUrl as "QUrl" {
            return SailfishApp::pathToMainQml();
        })
    }

    pub fn main() -> i32 {
        let args: Vec<CString> = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect();

        let argv: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();
        let mut argc: Box<c_int> = Box::new(args.len() as c_int);
        let argv_ptr = argv.as_ptr();
        let argc_ptr: *mut c_int = &mut *argc;

        cpp!(unsafe [argc_ptr as "int*", argv_ptr as "char**"] -> i32 as "int" {
            return SailfishApp::main(*argc_ptr, argv_ptr);
        })
    }
}

impl QApplicationFactory for SailfishApp {
    type ApplicationType = QGuiApplication;

    unsafe fn create_app(argc: *mut i32, argv: *const *const c_char) -> *mut QGuiApplication {
        cpp!([argc as "int*", argv as "char**"] -> *mut QGuiApplication as "QGuiApplication*" {
            return SailfishApp::application(*argc, argv);
        })
    }
}
