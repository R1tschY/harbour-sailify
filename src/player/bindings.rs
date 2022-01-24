//! C bindings for SailifyPlayer

use std::ffi::c_void;
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::sync::Arc;

use crate::player::error::LibrespotError;
use crate::player::events::{LibrespotEvent, LibrespotEventListener};
use crate::player::SailifyPlayer;

#[repr(C)]
#[derive(Clone)]
pub struct SailifyStringView<'a> {
    pub ptr: *const c_char,
    pub len: usize,
    _1: PhantomData<&'a c_char>,
}

impl<'a> SailifyStringView<'a> {
    pub fn new(ptr: *const c_char, len: usize) -> Self {
        Self {
            ptr,
            len,
            _1: PhantomData,
        }
    }
}

impl<'a> From<&str> for SailifyStringView<'a> {
    fn from(value: &str) -> Self {
        Self::new(value.as_ptr().cast::<c_char>(), value.len())
    }
}

trait ToFfi {
    type Ffi;
    fn to_ffi(&self) -> Self::Ffi;
}

trait IntoFfi {
    type Ffi;
    fn into_ffi(self) -> Self::Ffi;
}

impl<'a> ToFfi for &'a String {
    type Ffi = SailifyStringView<'a>;

    fn to_ffi(&self) -> SailifyStringView<'a> {
        SailifyStringView::from(*self as &'a str)
    }
}

impl<'a> ToFfi for &'a str {
    type Ffi = SailifyStringView<'a>;

    fn to_ffi(&self) -> SailifyStringView<'a> {
        SailifyStringView::from(*self)
    }
}

impl<'a> ToFfi for Option<&'a str> {
    type Ffi = SailifyStringView<'a>;

    fn to_ffi(&self) -> SailifyStringView<'a> {
        match &self {
            Some(value) => SailifyStringView::from(*value),
            None => SailifyStringView::new(std::ptr::null(), 0),
        }
    }
}

trait ToInternal {
    type Item;
    fn to_internal(&self) -> Self::Item;
}

impl<'a> ToInternal for SailifyStringView<'a> {
    type Item = Option<&'a str>;

    fn to_internal(&self) -> Self::Item {
        if self.ptr.is_null() {
            None
        } else {
            unsafe {
                Some(std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                    self.ptr.cast::<u8>(),
                    self.len,
                )))
            }
        }
    }
}

fn string_to_ffi(s: &str) -> SailifyStringView {
    SailifyStringView::from(s)
}

// SailifyString

pub struct SailifyString {
    __private: [u8; 0],
}

impl<'a> IntoFfi for String {
    type Ffi = *mut SailifyString;

    fn into_ffi(self) -> *mut SailifyString {
        Box::into_raw(self.into_boxed_str()).cast::<SailifyString>()
    }
}

impl<'a> IntoFfi for Option<String> {
    type Ffi = *mut SailifyString;

    fn into_ffi(self) -> *mut SailifyString {
        match self {
            Some(value) => value.into_ffi(),
            None => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sailify_string_delete(this: *mut SailifyString) {
    Box::from_raw(this);
}

// SailifyPlayer

#[no_mangle]
pub extern "C" fn sailify_player_new(callbacks: &SailifyCallback) -> *mut SailifyPlayer {
    Box::into_raw(Box::new(SailifyPlayer::new(Arc::new(callbacks.clone()))))
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_delete(this: *mut SailifyPlayer) {
    Box::from_raw(this);
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_play(this: &mut SailifyPlayer) {
    this.play();
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_pause(this: &mut SailifyPlayer) {
    this.pause();
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_next(this: &mut SailifyPlayer) {
    this.next();
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_previous(this: &mut SailifyPlayer) {
    this.previous();
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_stop(this: &mut SailifyPlayer) {
    this.stop();
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_start(this: &mut SailifyPlayer) {
    this.start();
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_logout(this: &mut SailifyPlayer) {
    this.logout();
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_set_username(
    this: &mut SailifyPlayer,
    username: SailifyStringView,
) {
    this.set_username(username.to_internal());
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_get_username(
    this: &mut SailifyPlayer,
) -> SailifyStringView {
    this.username().to_ffi()
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_set_password(
    this: &mut SailifyPlayer,
    password: SailifyStringView,
) {
    this.set_password(password.to_internal());
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_get_password(
    this: &mut SailifyPlayer,
) -> SailifyStringView {
    this.password().to_ffi()
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_is_active(this: &mut SailifyPlayer) -> bool {
    this.is_active()
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_refresh_access_token(this: &mut SailifyPlayer) {
    this.refresh_access_token();
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_get_device_id(
    this: &mut SailifyPlayer,
) -> SailifyStringView {
    this.device_id().to_ffi()
}

#[no_mangle]
pub unsafe extern "C" fn sailify_player_get_device_name(
    this: &mut SailifyPlayer,
) -> SailifyStringView {
    this.device_name().to_ffi()
}

// SailifyCallback

#[repr(C)]
pub enum SailifyErrorKind {
    MissingCredentials,
    IllegalConfig,
    Io,
    Connection,
    Panic,
    Token,
}

#[repr(C)]
#[derive(Clone)]
pub struct SailifyCallback {
    user_data: *mut c_void,

    stopped: unsafe fn(user_data: *mut c_void, play_request_id: u64, track_id: SailifyStringView),
    changed: unsafe fn(user_data: *mut c_void, new_track_id: SailifyStringView),
    loading: unsafe fn(
        user_data: *mut c_void,
        play_request_id: u64,
        track_id: SailifyStringView,
        position_ms: u32,
    ),
    playing: unsafe fn(
        user_data: *mut c_void,
        play_request_id: u64,
        track_id: SailifyStringView,
        position_ms: u32,
        duration_ms: u32,
    ),
    paused: unsafe fn(
        user_data: *mut c_void,
        play_request_id: u64,
        track_id: SailifyStringView,
        position_ms: u32,
        duration_ms: u32,
    ),
    unavailable:
        unsafe fn(user_data: *mut c_void, play_request_id: u64, track_id: SailifyStringView),
    volume_changed: unsafe fn(user_data: *mut c_void, value: u16),
    connecting: unsafe fn(user_data: *mut c_void),
    connected: unsafe fn(user_data: *mut c_void),
    error: unsafe fn(user_data: *mut c_void, kind: SailifyErrorKind, message: SailifyStringView),
    shutdown: unsafe fn(user_data: *mut c_void),
    start_reconnect: unsafe fn(user_data: *mut c_void),

    token_changed:
        unsafe fn(user_data: *mut c_void, access_token: SailifyStringView, expires_in: u32),

    destroy: unsafe fn(data: *mut c_void),
}

unsafe impl Send for SailifyCallback {}
unsafe impl Sync for SailifyCallback {}

impl Drop for SailifyCallback {
    fn drop(&mut self) {
        unsafe {
            (self.destroy)(self.user_data);
        }
    }
}

impl LibrespotEventListener for SailifyCallback {
    fn notify(&self, evt: LibrespotEvent) {
        unsafe {
            match evt {
                LibrespotEvent::Stopped {
                    play_request_id,
                    track_id,
                } => {
                    (self.stopped)(self.user_data, play_request_id, string_to_ffi(&track_id));
                }
                LibrespotEvent::Changed { new_track_id } => {
                    (self.changed)(self.user_data, string_to_ffi(&new_track_id));
                }
                LibrespotEvent::Loading {
                    play_request_id,
                    track_id,
                    position_ms,
                } => {
                    (self.loading)(
                        self.user_data,
                        play_request_id,
                        string_to_ffi(&track_id),
                        position_ms,
                    );
                }
                LibrespotEvent::Playing {
                    play_request_id,
                    track_id,
                    position_ms,
                    duration_ms,
                } => {
                    (self.playing)(
                        self.user_data,
                        play_request_id,
                        string_to_ffi(&track_id),
                        position_ms,
                        duration_ms,
                    );
                }
                LibrespotEvent::Paused {
                    play_request_id,
                    track_id,
                    position_ms,
                    duration_ms,
                } => {
                    (self.paused)(
                        self.user_data,
                        play_request_id,
                        string_to_ffi(&track_id),
                        position_ms,
                        duration_ms,
                    );
                }
                LibrespotEvent::Unavailable {
                    play_request_id,
                    track_id,
                } => {
                    (self.unavailable)(self.user_data, play_request_id, string_to_ffi(&track_id));
                }
                LibrespotEvent::VolumeSet { volume } => {
                    (self.volume_changed)(self.user_data, volume);
                }
                LibrespotEvent::Connecting => {
                    (self.connecting)(self.user_data);
                }
                LibrespotEvent::Connected => {
                    (self.connected)(self.user_data);
                }
                LibrespotEvent::ConnectionError { message } => {
                    (self.error)(
                        self.user_data,
                        SailifyErrorKind::Connection,
                        string_to_ffi(&message),
                    );
                }
                LibrespotEvent::Shutdown => {
                    (self.shutdown)(self.user_data);
                }
                LibrespotEvent::StartReconnect => {
                    (self.start_reconnect)(self.user_data);
                }
                LibrespotEvent::TokenChanged { token: result } => match result {
                    Ok(token) => (self.token_changed)(
                        self.user_data,
                        string_to_ffi(&token.access_token),
                        token.expires_in,
                    ),
                    Err(err) => {
                        (self.error)(self.user_data, SailifyErrorKind::Token, string_to_ffi(&err));
                    }
                },
                LibrespotEvent::Error { err } => {
                    let kind = match &err {
                        LibrespotError::MissingCredentials => SailifyErrorKind::MissingCredentials,
                        LibrespotError::IllegalConfig(_) => SailifyErrorKind::IllegalConfig,
                        LibrespotError::Io(_) => SailifyErrorKind::Io,
                        LibrespotError::Connection(_) => SailifyErrorKind::Connection,
                        LibrespotError::Panic(_) => SailifyErrorKind::Panic,
                    };
                    let error_string = format!("{}", &err);
                    (self.error)(self.user_data, kind, string_to_ffi(&error_string));
                }
                LibrespotEvent::Panic { message } => {
                    (self.error)(
                        self.user_data,
                        SailifyErrorKind::Panic,
                        string_to_ffi(&message),
                    );
                }
            }
        }
    }
}
