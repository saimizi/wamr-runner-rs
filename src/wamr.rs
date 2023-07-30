#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(unused)]
mod libiwasm;

#[allow(unused)]
use {
    super::error::WamrError,
    core::num::NonZeroUsize,
    error_stack::{IntoReport, Report, Result, ResultExt},
    jlogger_tracing::{
        jdebug, jerror, jinfo, jtrace, jwarn, JloggerBuilder, LevelFilter, LogTimeFormat,
    },
    std::{
        ffi::{CStr, CString},
        fs::{self, File},
        os::fd::AsRawFd,
        ptr,
    },
};

pub struct Wamr {}

struct MapBuffer {
    buf: *mut libc::c_void,
    len: libc::size_t,
}

impl Drop for MapBuffer {
    fn drop(&mut self) {
        if self.len != 0 && !self.buf.is_null() {
            unsafe {
                libc::munmap(self.buf, self.len);
            }
        }
    }
}

impl Default for MapBuffer {
    fn default() -> Self {
        Self {
            buf: ptr::null_mut(),
            len: 0,
        }
    }
}

#[allow(unused)]
impl MapBuffer {
    pub fn map_file(
        addr: *mut libc::c_void,
        len: libc::size_t,
        prot: libc::c_int,
        flags: libc::c_int,
        fd: libc::c_int,
        offset: libc::off_t,
    ) -> Result<Self, WamrError> {
        let buf = unsafe { libc::mmap(addr, len, prot, flags, fd, offset) };
        if buf == libc::MAP_FAILED {
            let errno = unsafe { *libc::__errno_location() };
            Err(WamrError::InvalidVal)
                .into_report()
                .attach_printable(format!("mmap() failed ({}) ", errno))
        } else {
            jdebug!("mmap() succeed: buf: {:p} size: {}", buf, len);
            Ok(Self { buf, len })
        }
    }

    pub fn buf_mut(&mut self) -> *mut libc::c_void {
        self.buf
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

const ERROR_BUF_LEN: u32 = 1024;

impl Wamr {
    pub fn run(wasm: &str) -> Result<(), WamrError> {
        /*
                let mut result = unsafe { libiwasm::wasm_runtime_init() };
                if !result {
                    return Err(WamrError::WamrErr)
                        .into_report()
                        .attach_printable("Failed to initialize wasm runtime");
                }
        */
        let mut init_args = libiwasm::RuntimeInitArgs {
            mem_alloc_type: libiwasm::mem_alloc_type_t_Alloc_With_System_Allocator,
            mem_alloc_option: libiwasm::MemAllocOption {
                pool: libiwasm::MemAllocOption__bindgen_ty_1 {
                    heap_buf: ptr::null_mut(),
                    heap_size: 0,
                },
            },
            native_module_name: ptr::null(),
            native_symbols: ptr::null_mut(),
            n_native_symbols: 0,
            max_thread_num: 0,
            ip_addr: [0; 128usize],
            unused: 0,
            instance_port: 0,
            fast_jit_code_cache_size: 0,
            running_mode: 0,
            llvm_jit_opt_level: 0,
            llvm_jit_size_level: 0,
        };

        let mut result = unsafe {
            libiwasm::wasm_runtime_full_init(&mut init_args as *mut libiwasm::RuntimeInitArgs)
        };
        if !result {
            return Err(WamrError::WamrErr)
                .into_report()
                .attach_printable("Failed to set init args for wasm runtime");
        }

        let mut buf = fs::read(wasm)
            .into_report()
            .change_context(WamrError::IOErr)
            .attach_printable(format!("Failed to read {}", wasm))?;

        let buf_size = buf.len() as u32;
        jdebug!("file size: {}", buf_size);
        let buf = buf.as_mut_ptr();
        let mut error_buf: [libc::c_char; ERROR_BUF_LEN as usize] = [0; ERROR_BUF_LEN as usize];
        let mut mb = MapBuffer::default();

        let ret = unsafe { libiwasm::wasm_runtime_is_xip_file(buf as *mut u8, buf_size) };
        if ret {
            let file = File::open(wasm)
                .into_report()
                .change_context(WamrError::InvalidVal)?;

            jinfo!("{} is a AOT XIP file", wasm);

            mb = MapBuffer::map_file(
                ptr::null_mut(),
                buf_size as usize,
                libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
                libc::MAP_32BIT | libc::MAP_PRIVATE,
                file.as_raw_fd(),
                0,
            )?;
        }

        let module = unsafe {
            if !mb.buf_mut().is_null() {
                libiwasm::wasm_runtime_load(
                    mb.buf_mut() as *mut u8,
                    buf_size,
                    &mut error_buf as *mut libc::c_char,
                    ERROR_BUF_LEN,
                )
            } else {
                libiwasm::wasm_runtime_load(
                    buf,
                    buf_size,
                    &mut error_buf as *mut libc::c_char,
                    ERROR_BUF_LEN,
                )
            }
        };

        //drop(mb);
        //drop(buf);

        if module.is_null() {
            let error_str = unsafe {
                CStr::from_ptr(error_buf.as_ptr())
                    .to_str()
                    .into_report()
                    .change_context(WamrError::InvalidVal)?
            };
            return Err(WamrError::WamrErr)
                .into_report()
                .attach_printable(error_str);
        }

        let module_inst = unsafe {
            libiwasm::wasm_runtime_instantiate(
                module,
                4 * 1024,
                4 * 1024,
                &mut error_buf as *mut libc::c_char,
                ERROR_BUF_LEN,
            )
        };

        if module_inst.is_null() {
            let error_str = unsafe {
                CStr::from_ptr(error_buf.as_ptr())
                    .to_str()
                    .into_report()
                    .change_context(WamrError::InvalidVal)?
            };
            return Err(WamrError::WamrErr)
                .into_report()
                .attach_printable(error_str);
        }

        jinfo!("Start running {}", wasm);

        result =
            unsafe { libiwasm::wasm_application_execute_main(module_inst, 0, ptr::null_mut()) };

        if !result {
            let exception = unsafe { libiwasm::wasm_runtime_get_exception(module_inst) };
            let error_str = unsafe {
                CStr::from_ptr(exception)
                    .to_str()
                    .into_report()
                    .change_context(WamrError::InvalidVal)?
            };

            return Err(WamrError::WamrErr)
                .into_report()
                .attach_printable(error_str);
        }

        Ok(())
    }
}
