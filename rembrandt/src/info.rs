// You can use below imports (and the warnings 
// about them being unused) as hints
use std::{
    ffi::{c_uint, CStr},
    path::Path,
    str::Utf8Error,
};

use image::{DynamicImage, ImageError};

impl ImageInfo {
    pub fn read_from_path(path: &CStr) -> Result<Self, ImageInfoError> {
        let path = Path::new(path.to_str()?);

        let img = image::open(path)?;

        let (color_type, bit_depth) = match img {
            DynamicImage::ImageLuma8(_) => (ColorType::Grayscale, BitDepth::Depth8),
            DynamicImage::ImageLumaA8(_) => (ColorType::GrayscaleAlpha, BitDepth::Depth8),
            DynamicImage::ImageRgb8(_) => (ColorType::Rgb, BitDepth::Depth8),
            DynamicImage::ImageRgba8(_) => (ColorType::RgbAlpha, BitDepth::Depth8),
            DynamicImage::ImageLuma16(_) => (ColorType::Grayscale, BitDepth::Depth16),
            DynamicImage::ImageLumaA16(_) => (ColorType::GrayscaleAlpha, BitDepth::Depth16),
            DynamicImage::ImageRgb16(_) => (ColorType::Rgb, BitDepth::Depth16),
            DynamicImage::ImageRgba16(_) => (ColorType::RgbAlpha, BitDepth::Depth16),
            _ => return Err(ImageInfoError::UnsupportedPixelType),
        };

        let info = ImageInfo {
            width: img.width(),
            height: img.height(),
            color_type: color_type.into(),
            bit_depth: bit_depth.into(),
        };

        Ok(info)
    }
}

// #[cfg(never)]
mod ffi {
    use std::ffi::{c_char, c_int, c_uint, CStr};

    use crate::info::ImageInfo;

    #[no_mangle]
    /// Open the file at the given `path`, and read its [ImageInfo].
    ///
    ///
    /// # Safety
    /// Callers need to take into account the safety rules of [CStr::from_ptr],
    /// as this function attempts to transform the path into a [CStr].
    /// Furthermore, the `image` 
    pub unsafe extern "C" fn image_info(path: *const c_char, img_info: *mut ImageInfo) -> c_int {
        if path.is_null() {
            return -1;
        }

        if img_info.is_null() {
            return -1;
        }

        // Check null-terminated
        let path = unsafe { CStr::from_ptr(path) };

        let i = match ImageInfo::read_from_path(path) {
            Ok(info) => info,
            Err(_) => return -1,
        };

        unsafe { *img_info = i };

        return 0;
    }

    #[no_mangle]
    pub extern "C" fn image_buf_size(info: ImageInfo) -> c_uint {
        info.height * image_row_size(info)
    }

    #[no_mangle]
    pub extern "C" fn image_row_size(info: ImageInfo) -> c_uint {
        info.width * info.color_type
    }
}

#[allow(dead_code)]
pub enum ImageInfoError {
    Utf8(Utf8Error),
    Image(ImageError),
    UnsupportedPixelType,
}

impl From<Utf8Error> for ImageInfoError {
    fn from(e: Utf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl From<ImageError> for ImageInfoError {
    fn from(e: ImageError) -> Self {
        Self::Image(e)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageInfo {
    width: c_uint,
    height: c_uint,
    color_type: c_uint,
    bit_depth: c_uint,
}

#[repr(u8)]
pub enum ColorType {
    Grayscale = 0,
    Rgb = 2,
    Palette = 3,
    GrayscaleAlpha = 4,
    RgbAlpha = 6,
}

impl From<ColorType> for c_uint {
    fn from(value: ColorType) -> Self {
        (value as u8).into()
    }
}

#[repr(u8)]
pub enum BitDepth {
    Depth1 = 1,
    Depth2 = 2,
    Depth4 = 4,
    Depth8 = 8,
    Depth16 = 16,
}

impl From<BitDepth> for c_uint {
    fn from(value: BitDepth) -> Self {
        (value as u8).into()
    }
}
