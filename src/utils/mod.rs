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
