use std::ffi::CString;

use ghw_sys::{
    ghw_close, ghw_disp_hie, ghw_disp_types, ghw_disp_values, ghw_handler, ghw_open, ghw_read_base,
    ghw_read_section,
};

use crate::{hierarchy::GHWHierarchy, section::GHWSection};

pub struct GHWHandle {
    handle: std::mem::MaybeUninit<ghw_handler>,
    filename: Option<String>,
}

impl GHWHandle {
    pub fn new() -> Self {
        GHWHandle {
            handle: std::mem::MaybeUninit::uninit(),
            filename: None,
        }
    }

    pub fn from_file(filename: &str) -> Self {
        let mut h = GHWHandle::new();
        h.filename = Some(filename.to_string());
        let path = CString::new(filename).unwrap();
        let ptr = path.as_ptr();
        unsafe {
            ghw_open(h.handle.as_mut_ptr(), ptr);
            h
        }
    }

    pub fn from_file_verbose(filename: &str) -> Self {
        let mut h = GHWHandle::new();
        h.set_verbose(true);
        h.filename = Some(filename.to_string());
        let path = CString::new(filename).unwrap();
        let ptr = path.as_ptr();
        unsafe {
            ghw_open(h.handle.as_mut_ptr(), ptr);
            h
        }
    }

    pub fn read_base(&mut self) -> i32 {
        unsafe { ghw_read_base(self.handle.as_mut_ptr()) }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        unsafe {
            (*self.handle.as_mut_ptr()).flag_verbose = verbose as i32;
        }
    }

    pub fn disp_values(&mut self) {
        unsafe {
            ghw_disp_values(self.handle.as_mut_ptr());
        }
    }

    pub fn disp_types(&mut self) {
        unsafe {
            ghw_disp_types(self.handle.as_mut_ptr());
        }
    }

    pub fn close(&mut self) {
        unsafe {
            ghw_close(self.handle.as_mut_ptr());
        }
    }

    pub fn hierarchy(&mut self) -> GHWHierarchy {
        unsafe {
            GHWHierarchy {
                handle: (*self.handle.as_mut_ptr()).hie,
            }
        }
    }

    pub fn disp_hierarchy(&mut self, hierarchy: GHWHierarchy) {
        unsafe {
            ghw_disp_hie(self.handle.as_mut_ptr(), hierarchy.handle);
        }
    }

    pub fn set_full_names(&mut self, full_names: bool) {
        unsafe {
            (*self.handle.as_mut_ptr()).flag_full_names = full_names as i32;
        }
    }

    pub fn read_section(&mut self) -> GHWSection {
        unsafe { ghw_read_section(self.handle.as_mut_ptr()).into() }
    }

    pub fn number_of_signals(&mut self) -> u32 {
        unsafe { (*self.handle.as_mut_ptr()).nbr_sigs }
    }

    pub fn number_of_strings(&mut self) -> u32 {
        unsafe { (*self.handle.as_mut_ptr()).nbr_str }
    }

    pub fn number_of_types(&mut self) -> u32 {
        unsafe { (*self.handle.as_mut_ptr()).nbr_types }
    }

    pub fn next_time(&mut self) -> i64 {
        unsafe { (*self.handle.as_mut_ptr()).snap_time }
    }
}
