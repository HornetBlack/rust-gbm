
#[macro_use]
extern crate bitflags;
extern crate libc;
#[cfg(feature = "egl_platform")]
extern crate egl;
#[cfg(feature = "wl_server")]
extern crate wayland_server;

pub mod ffi;

pub use ffi::FourCC;

#[allow(unused_imports)]
#[cfg(feature = "wl_server")]
use wayland_server::protocol::wl_buffer::WlBuffer;
#[allow(unused_imports)]
#[cfg(feature = "wl_server")]
use wayland_server::Resource;

use libc::c_void;
use std::os::unix::prelude::*;
use std::ffi::CStr;
use std::io;
use std::any::Any;
use std::mem::forget;
use std::fmt;

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
pub enum Error {
    DeviceCreation,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", <Self as std::error::Error>::description(self))
    }
}
impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self  {
            Error::DeviceCreation => "Failed to create gbm_device",
        }
    }
}

pub type EGLImageKHR = *mut c_void;

type Handle = u32;

#[derive(Debug)]
pub struct Device {
    ptr: *mut ffi::gbm_device,
}
#[derive(Debug)]
pub struct Bo {
    ptr: *mut ffi::gbm_bo,
}
#[derive(Debug)]
pub struct Surface {
    ptr: *mut ffi::gbm_surface,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BoFormat {
    XRGB8888,
    ARGB8888,
    // FourCC(FourCC)
}
impl BoFormat {
    fn as_u32(&self) -> u32 {
        match *self {
            BoFormat::XRGB8888 => ffi::gbm_bo_format::GBM_BO_FORMAT_XRGB8888 as u32,
            BoFormat::ARGB8888 => ffi::gbm_bo_format::GBM_BO_FORMAT_ARGB8888 as u32,
            // BoFormat::FourCC(fcc) => fcc as u32
        }
    }
    fn from_u32(u: u32) -> Option<BoFormat> {
        if u == ffi::gbm_bo_format::GBM_BO_FORMAT_XRGB8888 as u32 {
            Some(BoFormat::XRGB8888)
        } else if u == ffi::gbm_bo_format::GBM_BO_FORMAT_ARGB8888 as u32 {
            Some(BoFormat::ARGB8888)
        } else {
            // TODO: Add other formats. FourCC formats should be supported
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct BoFlags {
    flags: ffi::gbm_bo_flags,
}
impl BoFlags {
    pub fn new() -> BoFlags {
        BoFlags {
            flags: ffi::gbm_bo_flags::empty()
        }
    }
    pub fn scanout(&mut self, v: bool) -> &mut BoFlags {
        self.set(ffi::GBM_BO_USE_SCANOUT, v)
    }
    pub fn cursor(&mut self, v: bool) -> &mut BoFlags {
        self.set(ffi::GBM_BO_USE_CURSOR, v)
    }
    pub fn rendering(&mut self, v: bool) -> &mut BoFlags {
        self.set(ffi::GBM_BO_USE_RENDERING, v)
    }
    pub fn write(&mut self, v: bool) -> &mut BoFlags {
        self.set(ffi::GBM_BO_USE_WRITE, v)
    }
    pub fn linear(&mut self, v: bool) -> &mut BoFlags {
        self.set(ffi::GBM_BO_USE_LINEAR, v)
    }

    fn as_u32(&self) -> u32 {
        self.flags.bits()
    }
    fn set(&mut self, flag: ffi::gbm_bo_flags, v: bool) -> &mut BoFlags {
        if v { self.flags.insert(flag) } else { self.flags.remove(flag) }
        self
    }
}

impl Device {
    unsafe fn from_ptr(ptr: *mut ffi::gbm_device) -> Option<Device> {
        if ptr.is_null() { None } else { Some(Device { ptr: ptr }) }
    }
    pub fn as_ptr(&mut self) -> *mut ffi::gbm_device {
        self.ptr
    }

    #[cfg(feature = "egl_platform")]
    pub fn as_egl_display(&self) -> egl::EGLNativeDisplayType {
        self.ptr as *mut _
    }

    pub fn create(fd: RawFd) -> Result<Device, Error> {
        unsafe {
            match Device::from_ptr(ffi::gbm_create_device(fd)) {
                None => Err(Error::DeviceCreation),
                Some(dev) => Ok(dev),
            }
        }
    }

    pub fn get_fd(&self) -> RawFd {
        unsafe { ffi::gbm_device_get_fd(self.ptr) }
    }

    pub fn get_backend_name_cstr(&self) -> &'static CStr {
        unsafe {
            CStr::from_ptr(ffi::gbm_device_get_backend_name(self.ptr))
        }
    }

    pub fn get_backend_name(&self) -> &'static str {
        self.get_backend_name_cstr().to_str().unwrap()
    }
    
    pub fn is_format_supported(&self, format: BoFormat, flags: BoFlags) -> bool {
        unsafe {
            ffi::gbm_device_is_format_supported(self.ptr, format.as_u32(),
                                                flags.as_u32()) != 0
        }
    }

    pub fn surface_create(&mut self,
                          width: u32, height: u32,
                          format: BoFormat, flags: BoFlags) -> Surface
    {
        unsafe {
            Surface::from_ptr(ffi::gbm_surface_create(self.ptr,
                                                      width, height,
                                                      format.as_u32(),
                                                      flags.as_u32()))
                .unwrap()
        }
    }

    pub fn bo_create(&mut self,
                     width: u32, height: u32,
                     format: BoFormat, flags: BoFlags) -> Bo
    {
        unsafe {
            Bo::from_ptr(ffi::gbm_bo_create(self.ptr,
                                            width, height,
                                            format.as_u32(),
                                            flags.as_u32()))
                .unwrap()
        }
    }

    pub fn bo_import(&self, bo: BoImport, usage: BoFlags) -> Result<Bo, ()>
    {
        let bo = unsafe { match bo {
            BoImport::Fd{ fd, width, height, stride, format } => {
                let mut dmabuf = ffi::gbm_import_fd_data {
                    fd: fd, width: width, height: height,
                    stride: stride, format: format.as_u32()
                };
                ffi::gbm_bo_import(self.ptr, ffi::GBM_BO_IMPORT_FD,
                                   &mut dmabuf as *mut _ as *mut _,
                                   usage.as_u32())
            }
            #[cfg(feature = "wl_server")]
            BoImport::WlBuffer(wl_buffer) => {
                ffi::gbm_bo_import(self.ptr, ffi::GBM_BO_IMPORT_WL_BUFFER,
                              wl_buffer.ptr() as *mut c_void, usage.as_u32())
            }
            #[cfg(feature = "egl_platform")]
            BoImport::EglImage(egl_image) => {
                ffi::gbm_bo_import(self.ptr, ffi::GBM_BO_IMPORT_EGL_IMAGE,
                              egl_image, usage.as_u32())
            }
        } };
        if bo.is_null() {
            Err(())
        } else {
            Ok(Bo { ptr: bo })
        }
    }

}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe { ffi::gbm_device_destroy(self.ptr) }
    }
}

pub enum BoImport {
    Fd { fd: RawFd, width: u32, height: u32, stride: u32, format: BoFormat },
    #[cfg(feature = "wl_server")]
    WlBuffer(WlBuffer),
    #[cfg(feature = "egl_platform")]
    EglImage(EGLImageKHR),
}

impl Bo {
    unsafe fn from_ptr(ptr: *mut ffi::gbm_bo) -> Option<Bo> {
        if ptr.is_null() { None } else { Some(Bo { ptr: ptr }) }
    }

    #[cfg(feature = "egl_platform")]
    pub fn as_egl_pixmap(&self) -> egl::EGLNativePixmapType {
        self.ptr as *mut _
    }
    
    pub fn get_width(&self) -> u32 {
        unsafe { ffi::gbm_bo_get_width(self.ptr) }
    }
    pub fn get_height(&self) -> u32 {
        unsafe { ffi::gbm_bo_get_height(self.ptr) }
    }
    pub fn get_stride(&self) -> u32 {
        unsafe { ffi::gbm_bo_get_stride(self.ptr) }
    }
    pub fn get_format(&self) -> BoFormat {
        BoFormat::from_u32(unsafe {
            ffi::gbm_bo_get_format(self.ptr)
        }).unwrap()
    }
    pub fn get_device(&self) -> Device {
        unsafe {
            Device::from_ptr(ffi::gbm_bo_get_device(self.ptr))
                .unwrap()
        }
    }

    pub fn get_handle(&self) -> Handle {
        unsafe {
            ffi::gbm_bo_get_handle(self.ptr).u32()
        }
    }
    
    pub fn get_fd(&self) -> RawFd {
        unsafe {
            ffi::gbm_bo_get_fd(self.ptr)
        }
    }

    /// Set's user data.  Internally this is done by creating a
    /// `Box<Box<T>>` containing your data then setting up a callback
    /// that cast's the pointer given to use by libgbm back to a Box
    /// so that it will be dropped by Rust.
    ///
    /// This does not check if user data was already set, and will
    /// leak the already set user data.
    pub fn set_user_data<T:Any>(&mut self, data: T) {
        // TODO: This may involve some rework.  There might be a good
        // reason to instead point to some struct that contains the
        // `Box<Any> ` and some other useful data. (Perhaps a safe way to
        // check if the data is from Rust at all.)
        //
        #[repr(C)]
        struct _user_data {
            // Compare this to some global initialised pointer to
            // validate that this is rust data
            is_rust_check: *const u32,
            user_data: Box<Any>,
        };
        
        extern fn destroy_user_data(_bo: *mut ffi::gbm_bo, ptr: *mut c_void) {
            unsafe {
                let _b: Box<Box<Any>> = Box::from_raw(ptr as *mut _);
                // Rust will magically Drop for us!
            }
        }

        let b: Box<Box<Any>> = Box::new(Box::new(data) as Box<Any>);
        let ptr = Box::into_raw(b) as *mut c_void;
        let callback = destroy_user_data as ffi::BoCallback;
        unsafe {
            ffi::gbm_bo_set_user_data(self.ptr, ptr, callback)
        }
    }

    pub fn has_user_data(&self) -> bool {
        unsafe { !ffi::gbm_bo_get_user_data(self.ptr).is_null() }
    }
    
    /// Internally this is using Any::downcase_ref to check the
    /// underlying type.
    ///
    /// This will fail either because there is no
    /// data or the data is not the correct type.
    /// (The former can be checked using `has_user_data`)
    pub fn get_user_data<'a, T: Any>(&'a self) -> Option<&'a T> {
        unsafe {
            let ptr = ffi::gbm_bo_get_user_data(self.ptr) as *mut Box<Any>;
            if ptr.is_null() {
                return None
            } else {
                let r: &'a Box<Any> = &(*ptr);
                r.downcast_ref()
            }            
        }
    }
    pub fn get_user_data_mut<'a, T: Any>(&'a mut self) -> Option<&'a mut T> {
        unsafe {
            let ptr = ffi::gbm_bo_get_user_data(self.ptr) as *mut Box<Any>;
            if ptr.is_null() {
                return None
            } else {
                let r: &'a mut Box<Any> = &mut (*ptr);
                r.downcast_mut()
            }            
        }
    }
}

impl io::Write for Bo {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            let ptr = buf.as_ptr() as *const c_void;
            if ffi::gbm_bo_write(self.ptr, ptr, buf.len()) == -1 {
                Err(io::Error::last_os_error())
            } else {
                Ok(buf.len())
            }
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for Bo {
    fn drop(&mut self) {
        unsafe {
            ffi::gbm_bo_destroy(self.ptr)
        }
    }
}


impl Surface {
    unsafe fn from_ptr(ptr: *mut ffi::gbm_surface) -> Option<Surface> {
        if ptr.is_null() { None } else { Some(Surface { ptr: ptr }) }
    }

    #[cfg(feature = "egl_platform")]
    pub fn as_egl_window(&self) -> egl::EGLNativeWindowType {
        self.ptr as *mut _
    }
    
    pub fn needs_lock_front_buffer(&self) -> bool {
        unsafe { ffi::gbm_surface_needs_lock_front_buffer(self.ptr) != 0 }
    }
    
    pub fn lock_front_buffer(&mut self) -> Option<Bo> {
        unsafe {
            Bo::from_ptr(ffi::gbm_surface_lock_front_buffer(self.ptr))
        }
    }

    pub fn release_buffer(&mut self, bo: Bo) {
        unsafe {
            ffi::gbm_surface_release_buffer(self.ptr, bo.ptr);
            forget(bo);
        }
    }

    pub fn has_free_buffers(&self) -> bool {
        unsafe {
            ffi::gbm_surface_has_free_buffers(self.ptr) != 0
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            ffi::gbm_surface_destroy(self.ptr)
        }
    }
}


