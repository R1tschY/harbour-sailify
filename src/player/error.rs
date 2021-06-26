use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum LibrespotError {
        MissingCredentials {
            display("Credentials are missing")
        }

        IllegalConfig(msg: String) {
            display("Illegal configuration: {}", msg)
        }

        Io(err: std::io::Error) {
            from()
            display("I/O error: {}", err)
            source(err)
        }

        Connection(msg: String) {
            display("Connection error: {}", msg)
        }
    }
}

pub type LibrespotResult<T> = Result<T, LibrespotError>;
