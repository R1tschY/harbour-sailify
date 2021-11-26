use crate::player::qobject::SailifyPlayer;
use crate::player::qtgateway::{LibrespotEvent, LibrespotEventListener};
use std::ffi::c_void;
use std::os::raw::c_char;
use std::sync::Arc;

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

// SailifyCallback

#[repr(C)]
pub struct SailifyStringView {
    pub ptr: *const c_char,
    pub len: usize,
}

impl From<&str> for SailifyStringView {
    fn from(value: &str) -> Self {
        Self {
            ptr: value.as_ptr() as *const _ as *const c_char,
            len: value.len(),
        }
    }
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
                    (self.stopped)(self.user_data, play_request_id, (&track_id as &str).into());
                }
                LibrespotEvent::Changed { .. } => {}
                LibrespotEvent::Loading { .. } => {}
                LibrespotEvent::Playing { .. } => {}
                LibrespotEvent::Paused { .. } => {}
                LibrespotEvent::Unavailable { .. } => {}
                LibrespotEvent::VolumeSet { .. } => {}
                LibrespotEvent::Connecting => {}
                LibrespotEvent::Connected => {}
                LibrespotEvent::ConnectionError { .. } => {}
                LibrespotEvent::Shutdown => {}
                LibrespotEvent::StartReconnect => {}
                LibrespotEvent::TokenChanged { .. } => {}
                LibrespotEvent::Error => {}
                LibrespotEvent::Panic { .. } => {}
            }
        }
    }
}
