
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use libc::{
    c_void,
    c_char,
    c_int,
    size_t,
};
use std::mem::transmute;
use std::os::unix::io::RawFd;

pub type s32 = i32;
pub type s64 = i64;

pub enum gbm_device { }
pub enum gbm_bo { }
pub enum gbm_surface { }

/// Union in C.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gbm_bo_handle(u64);

impl gbm_bo_handle {
    pub unsafe fn ptr(self) -> *mut c_void { transmute(self.0 as usize) }
    pub unsafe fn s32(self) -> s32 { self.0 as s32 }
    pub unsafe fn u32(self) -> u32 { self.0 as u32 }
    pub unsafe fn s64(self) -> s64 { self.0 as s64 }
    pub unsafe fn u64(self) -> u64 { self.0 as u64 }
}

/// Format of the allocated buffer
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum gbm_bo_format {
    /// RGB with 8 bits per channel in a 32 bit value
    GBM_BO_FORMAT_XRGB8888 = 0, 
    /// ARGB with 8 bits per channel in a 32 bit value */
    GBM_BO_FORMAT_ARGB8888 = 1
}

macro_rules! gbm_fourcc_code {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        (($a as u32) | (($b as u32) << 8) |
	 (($c as u32) << 16) | (($d as u32) << 24))
    }
}

macro_rules! decl_gbm_fourcc_list {
    ( $( $name:ident ($a:expr, $b:expr, $c:expr, $d:expr) ),* ) => {
        #[repr(u32)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub enum FourCC {
            $(
                $name = gbm_fourcc_code!($a, $b, $c, $d),
            )*
        }
    }
}

decl_gbm_fourcc_list! {
    /* color index */
    C8 ('C', '8', ' ', ' '), /* [7:0] C */

    /* 8 bpp RGB */
    RGB332 ('R', 'G', 'B', '8'), /* [7:0] R:G:B 3:3:2 */
    BGR233 ('B', 'G', 'R', '8'), /* [7:0] B:G:R 2:3:3 */

    /* 16 bpp RGB */
    XRGB4444 ('X', 'R', '1', '2'), /* [15:0] x:R:G:B 4:4:4:4 little endian */
    XBGR4444 ('X', 'B', '1', '2'), /* [15:0] x:B:G:R 4:4:4:4 little endian */
    RGBX4444 ('R', 'X', '1', '2'), /* [15:0] R:G:B:x 4:4:4:4 little endian */
    BGRX4444 ('B', 'X', '1', '2'), /* [15:0] B:G:R:x 4:4:4:4 little endian */

    ARGB4444 ('A', 'R', '1', '2'), /* [15:0] A:R:G:B 4:4:4:4 little endian */
    ABGR4444 ('A', 'B', '1', '2'), /* [15:0] A:B:G:R 4:4:4:4 little endian */
    RGBA4444 ('R', 'A', '1', '2'), /* [15:0] R:G:B:A 4:4:4:4 little endian */
    BGRA4444 ('B', 'A', '1', '2'), /* [15:0] B:G:R:A 4:4:4:4 little endian */

    XRGB1555 ('X', 'R', '1', '5'), /* [15:0] x:R:G:B 1:5:5:5 little endian */
    XBGR1555 ('X', 'B', '1', '5'), /* [15:0] x:B:G:R 1:5:5:5 little endian */
    RGBX5551 ('R', 'X', '1', '5'), /* [15:0] R:G:B:x 5:5:5:1 little endian */
    BGRX5551 ('B', 'X', '1', '5'), /* [15:0] B:G:R:x 5:5:5:1 little endian */

    ARGB1555 ('A', 'R', '1', '5'), /* [15:0] A:R:G:B 1:5:5:5 little endian */
    ABGR1555 ('A', 'B', '1', '5'), /* [15:0] A:B:G:R 1:5:5:5 little endian */
    RGBA5551 ('R', 'A', '1', '5'), /* [15:0] R:G:B:A 5:5:5:1 little endian */
    BGRA5551 ('B', 'A', '1', '5'), /* [15:0] B:G:R:A 5:5:5:1 little endian */

    RGB565 ('R', 'G', '1', '6'), /* [15:0] R:G:B 5:6:5 little endian */
    BGR565 ('B', 'G', '1', '6'), /* [15:0] B:G:R 5:6:5 little endian */

    /* 24 bpp RGB */
    RGB888 ('R', 'G', '2', '4'), /* [23:0] R:G:B little endian */
    BGR888 ('B', 'G', '2', '4'), /* [23:0] B:G:R little endian */

    /* 32 bpp RGB */
    XRGB8888 ('X', 'R', '2', '4'), /* [31:0] x:R:G:B 8:8:8:8 little endian */
    XBGR8888 ('X', 'B', '2', '4'), /* [31:0] x:B:G:R 8:8:8:8 little endian */
    RGBX8888 ('R', 'X', '2', '4'), /* [31:0] R:G:B:x 8:8:8:8 little endian */
    BGRX8888 ('B', 'X', '2', '4'), /* [31:0] B:G:R:x 8:8:8:8 little endian */

    ARGB8888 ('A', 'R', '2', '4'), /* [31:0] A:R:G:B 8:8:8:8 little endian */
    ABGR8888 ('A', 'B', '2', '4'), /* [31:0] A:B:G:R 8:8:8:8 little endian */
    RGBA8888 ('R', 'A', '2', '4'), /* [31:0] R:G:B:A 8:8:8:8 little endian */
    BGRA8888 ('B', 'A', '2', '4'), /* [31:0] B:G:R:A 8:8:8:8 little endian */

    XRGB2101010 ('X', 'R', '3', '0'), /* [31:0] x:R:G:B 2:10:10:10 little endian */
    XBGR2101010 ('X', 'B', '3', '0'), /* [31:0] x:B:G:R 2:10:10:10 little endian */
    RGBX1010102 ('R', 'X', '3', '0'), /* [31:0] R:G:B:x 10:10:10:2 little endian */
    BGRX1010102 ('B', 'X', '3', '0'), /* [31:0] B:G:R:x 10:10:10:2 little endian */

    ARGB2101010 ('A', 'R', '3', '0'), /* [31:0] A:R:G:B 2:10:10:10 little endian */
    ABGR2101010 ('A', 'B', '3', '0'), /* [31:0] A:B:G:R 2:10:10:10 little endian */
    RGBA1010102 ('R', 'A', '3', '0'), /* [31:0] R:G:B:A 10:10:10:2 little endian */
    BGRA1010102 ('B', 'A', '3', '0'), /* [31:0] B:G:R:A 10:10:10:2 little endian */

    /* packed YCbCr */
    YUYV ('Y', 'U', 'Y', 'V'), /* [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian */
    YVYU ('Y', 'V', 'Y', 'U'), /* [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian */
    UYVY ('U', 'Y', 'V', 'Y'), /* [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian */
    VYUY ('V', 'Y', 'U', 'Y'), /* [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian */

    AYUV ('A', 'Y', 'U', 'V'), /* [31:0] A:Y:Cb:Cr 8:8:8:8 little endian */

    /*
     * 2 plane YCbCr
     * index 0 = Y plane, [7:0] Y
     * index 1 = Cr:Cb plane, [15:0] Cr:Cb little endian
     * or
     * index 1 = Cb:Cr plane, [15:0] Cb:Cr little endian
     */
    NV12 ('N', 'V', '1', '2'), /* 2x2 subsampled Cr:Cb plane */
    NV21 ('N', 'V', '2', '1'), /* 2x2 subsampled Cb:Cr plane */
    NV16 ('N', 'V', '1', '6'), /* 2x1 subsampled Cr:Cb plane */
    NV61 ('N', 'V', '6', '1'), /* 2x1 subsampled Cb:Cr plane */

    /*
     * 3 plane YCbCr
     * index 0: Y plane, [7:0] Y
     * index 1: Cb plane, [7:0] Cb
     * index 2: Cr plane, [7:0] Cr
     * or
     * index 1: Cr plane, [7:0] Cr
     * index 2: Cb plane, [7:0] Cb
     */
    YUV410 ('Y', 'U', 'V', '9'), /* 4x4 subsampled Cb (1) and Cr (2) planes */
    YVU410 ('Y', 'V', 'U', '9'), /* 4x4 subsampled Cr (1) and Cb (2) planes */
    YUV411 ('Y', 'U', '1', '1'), /* 4x1 subsampled Cb (1) and Cr (2) planes */
    YVU411 ('Y', 'V', '1', '1'), /* 4x1 subsampled Cr (1) and Cb (2) planes */
    YUV420 ('Y', 'U', '1', '2'), /* 2x2 subsampled Cb (1) and Cr (2) planes */
    YVU420 ('Y', 'V', '1', '2'), /* 2x2 subsampled Cr (1) and Cb (2) planes */
    YUV422 ('Y', 'U', '1', '6'), /* 2x1 subsampled Cb (1) and Cr (2) planes */
    YVU422 ('Y', 'V', '1', '6'), /* 2x1 subsampled Cr (1) and Cb (2) planes */
    YUV444 ('Y', 'U', '2', '4'), /* non-subsampled Cb (1) and Cr (2) planes */
    YVU444 ('Y', 'V', '2', '4') /* non-subsampled Cr (1) and Cb (2) planes */
}

   
bitflags! {
    pub flags gbm_bo_flags: u32 {
        const GBM_BO_USE_SCANOUT = (1 << 0),
        const GBM_BO_USE_CURSOR  = (1 << 1),
        const GBM_BO_USE_RENDERING = (1 << 2),
        const GBM_BO_USE_WRITE = (1 << 3),
        const GBM_BO_USE_LINEAR = (1 << 4),
    }
}

pub const GBM_BO_IMPORT_WL_BUFFER: u32 = 0x5501;
pub const GBM_BO_IMPORT_EGL_IMAGE: u32 = 0x5502;
pub const GBM_BO_IMPORT_FD: u32 = 0x5503;

#[repr(C)]
pub struct gbm_import_fd_data {
    pub fd: RawFd,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}

pub type BoCallback = extern fn(bo: *mut gbm_bo, data: *mut c_void);

#[link(name = "gbm")]
extern {
    pub fn gbm_device_get_fd(gbm: *mut gbm_device) -> RawFd;
    pub fn gbm_device_get_backend_name(gbm: *mut gbm_device) -> *const c_char;
    pub fn gbm_device_is_format_supported(gbm: *mut gbm_device,
                                          format: u32, usage: u32) -> c_int;
    pub fn gbm_device_destroy(gbm: *mut gbm_device);
    pub fn gbm_create_device(fd: RawFd) -> *mut gbm_device;

    pub fn gbm_bo_create(gbm: *mut gbm_device,
                         width: u32,  height: u32,
                         format: u32, flags: u32)
                         -> *mut gbm_bo;
    pub fn gbm_bo_import(gbm: *mut gbm_device, type_: u32,
                         buffer: *mut c_void, usage: u32) -> *mut gbm_bo;
    pub fn gbm_bo_get_width(bo: *mut gbm_bo) -> u32;
    pub fn gbm_bo_get_height(bo: *mut gbm_bo) -> u32;
    pub fn gbm_bo_get_stride(bo: *mut gbm_bo) -> u32;
    pub fn gbm_bo_get_format(bo: *mut gbm_bo) -> u32;
    pub fn gbm_bo_get_device(bo: *mut gbm_bo) -> *mut gbm_device;
    pub fn gbm_bo_get_handle(bo: *mut gbm_bo) -> gbm_bo_handle;
    pub fn gbm_bo_get_fd(bo: *mut gbm_bo) -> RawFd;
    pub fn gbm_bo_write(bo: *mut gbm_bo, buf: *const c_void, count: size_t) -> c_int;
    pub fn gbm_bo_set_user_data(bo: *mut gbm_bo, data: *mut c_void,
                                destroy_user_data: BoCallback);
    pub fn gbm_bo_get_user_data(bo: *mut gbm_bo) -> *mut c_void;
    pub fn gbm_bo_destroy(bo: *mut gbm_bo);
    pub fn gbm_surface_create(gbm: *mut gbm_device,
                              width: u32, height: u32,
		              format: u32, flags: u32) -> *mut gbm_surface;
    pub fn gbm_surface_needs_lock_front_buffer(surface: *mut gbm_surface) -> c_int;
    pub fn gbm_surface_lock_front_buffer(surface: *mut gbm_surface) -> *mut gbm_bo;
    pub fn gbm_surface_release_buffer(surface: *mut gbm_surface, bo: *mut gbm_bo);
    pub fn gbm_surface_has_free_buffers(surface: *mut gbm_surface) -> c_int;
    pub fn gbm_surface_destroy(surface: *mut gbm_surface);
}
