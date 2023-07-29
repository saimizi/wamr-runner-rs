#[allow(unused)]
use {
    error_stack::{IntoReport, Report, Result, ResultExt},
    jlogger_tracing::{
        jdebug, jerror, jinfo, jtrace, jwarn, JloggerBuilder, LevelFilter, LogTimeFormat,
    },
    std::{
        boxed::Box,
        ffi::{CStr, CString},
        fmt::Display,
        sync::atomic::{AtomicI32, Ordering},
    },
};

#[derive(Debug)]
pub enum WamrError {
    InvalidVal,
    WamrErr,
    IOErr,
}

impl Display for WamrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (code, desc) = error_desc(self);
        write!(f, "{}({}).", code, desc)
    }
}

impl std::error::Error for WamrError {}

pub fn error_desc(error: &WamrError) -> (i32, &'static str) {
    match error {
        WamrError::InvalidVal=> (-1, "Invalid value"),
        WamrError::WamrErr => (-2, "Wamr runtime error"),
        WamrError::IOErr => (-3, "IO error"),
    }
}
