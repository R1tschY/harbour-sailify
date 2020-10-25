include!(concat!(env!("OUT_DIR"), "/qffi_LibrespotGateway.rs"));

pub struct LibrespotGatewayPrivate {
    qobject: *mut LibrespotGateway,
}

impl LibrespotGatewayPrivate {
    pub fn new(qobject: *mut LibrespotGateway) -> Self {
        Self { qobject }
    }
}
