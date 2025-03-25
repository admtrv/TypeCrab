/*
 * core/src/response.rs
 */

#[derive(Debug, Copy, Clone)]
pub enum Level {
    Info,
    Warning,
    Error,
}

impl Level {
    pub fn escalate(&mut self, new: Level) {
        use Level::*;
        match (*self, new) {
            (Error, _) => {}
            (Warning, Error) => *self = Error,
            (Info, Error) => *self = Error,
            (Info, Warning) => *self = Warning,
            _ => {}
        }
    }
}


#[derive(Debug)]
pub struct Response<T> {
    pub payload: T,
    pub message: Option<(Level, String)>,
}

impl<T> Response<T> {
    pub fn plain(data: T) -> Self {
        Self {
            payload: data,
            message: None,
        }
    }

    pub fn with_info<S: Into<String>>(data: T, msg: S) -> Self {
        Self {
            payload: data,
            message: Some((Level::Info, msg.into())),
        }
    }

    pub fn with_warning<S: Into<String>>(data: T, msg: S) -> Self {
        Self {
            payload: data,
            message: Some((Level::Warning, msg.into())),
        }
    }

    pub fn with_error<S: Into<String>>(data: T, msg: S) -> Self {
        Self {
            payload: data,
            message: Some((Level::Error, msg.into())),
        }
    }
}
