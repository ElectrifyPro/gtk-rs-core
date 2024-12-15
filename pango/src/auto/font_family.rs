// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, FontFace};
#[cfg(feature = "v1_52")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_52")))]
use glib::signal::{connect_raw, SignalHandlerId};
use glib::{prelude::*, translate::*};
#[cfg(feature = "v1_52")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_52")))]
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "PangoFontFamily")]
    pub struct FontFamily(Object<ffi::PangoFontFamily, ffi::PangoFontFamilyClass>) @implements gio::ListModel;

    match fn {
        type_ => || ffi::pango_font_family_get_type(),
    }
}

impl FontFamily {
    pub const NONE: Option<&'static FontFamily> = None;
}

impl std::fmt::Display for FontFamily {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&FontFamilyExt::name(self))
    }
}

pub trait FontFamilyExt: IsA<FontFamily> + 'static {
    #[cfg(feature = "v1_46")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_font_family_get_face")]
    #[doc(alias = "get_face")]
    fn face(&self, name: Option<&str>) -> Option<FontFace> {
        unsafe {
            from_glib_none(ffi::pango_font_family_get_face(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_family_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::pango_font_family_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_family_is_monospace")]
    #[doc(alias = "is-monospace")]
    fn is_monospace(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_font_family_is_monospace(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_44")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_font_family_is_variable")]
    #[doc(alias = "is-variable")]
    fn is_variable(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_font_family_is_variable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_family_list_faces")]
    fn list_faces(&self) -> Vec<FontFace> {
        unsafe {
            let mut faces = std::ptr::null_mut();
            let mut n_faces = std::mem::MaybeUninit::uninit();
            ffi::pango_font_family_list_faces(
                self.as_ref().to_glib_none().0,
                &mut faces,
                n_faces.as_mut_ptr(),
            );
            FromGlibContainer::from_glib_container_num(faces, n_faces.assume_init() as _)
        }
    }

    #[cfg(feature = "v1_52")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_52")))]
    #[doc(alias = "is-monospace")]
    fn connect_is_monospace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_monospace_trampoline<
            P: IsA<FontFamily>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::PangoFontFamily,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FontFamily::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-monospace\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_is_monospace_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_52")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_52")))]
    #[doc(alias = "is-variable")]
    fn connect_is_variable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_variable_trampoline<
            P: IsA<FontFamily>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::PangoFontFamily,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FontFamily::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-variable\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_is_variable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_52")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_52")))]
    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<FontFamily>, F: Fn(&P) + 'static>(
            this: *mut ffi::PangoFontFamily,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FontFamily::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<FontFamily>> FontFamilyExt for O {}
