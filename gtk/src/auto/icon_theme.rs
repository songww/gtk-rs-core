// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use cairo;
use gdk;
use gdk_pixbuf;
use gio;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gtk_sys;
use std;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use IconInfo;
use IconLookupFlags;

glib_wrapper! {
    pub struct IconTheme(Object<gtk_sys::GtkIconTheme, gtk_sys::GtkIconThemeClass>);

    match fn {
        get_type => || gtk_sys::gtk_icon_theme_get_type(),
    }
}

impl IconTheme {
    pub fn new() -> IconTheme {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_icon_theme_new()) }
    }

    pub fn get_default() -> Option<IconTheme> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gtk_sys::gtk_icon_theme_get_default()) }
    }

    pub fn get_for_screen(screen: &gdk::Screen) -> Option<IconTheme> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gtk_sys::gtk_icon_theme_get_for_screen(
                screen.to_glib_none().0,
            ))
        }
    }
}

impl Default for IconTheme {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ICON_THEME: Option<&IconTheme> = None;

pub trait IconThemeExt: 'static {
    fn add_resource_path(&self, path: &str);

    fn append_search_path<P: AsRef<std::path::Path>>(&self, path: P);

    fn get_example_icon_name(&self) -> Option<GString>;

    fn has_icon(&self, icon_name: &str) -> bool;

    fn list_contexts(&self) -> Vec<GString>;

    fn list_icons(&self, context: Option<&str>) -> Vec<GString>;

    fn load_icon(
        &self,
        icon_name: &str,
        size: i32,
        flags: IconLookupFlags,
    ) -> Result<Option<gdk_pixbuf::Pixbuf>, glib::Error>;

    fn load_icon_for_scale(
        &self,
        icon_name: &str,
        size: i32,
        scale: i32,
        flags: IconLookupFlags,
    ) -> Result<Option<gdk_pixbuf::Pixbuf>, glib::Error>;

    fn load_surface<P: IsA<gdk::Window>>(
        &self,
        icon_name: &str,
        size: i32,
        scale: i32,
        for_window: Option<&P>,
        flags: IconLookupFlags,
    ) -> Result<Option<cairo::Surface>, glib::Error>;

    fn lookup_by_gicon<P: IsA<gio::Icon>>(
        &self,
        icon: &P,
        size: i32,
        flags: IconLookupFlags,
    ) -> Option<IconInfo>;

    fn lookup_by_gicon_for_scale<P: IsA<gio::Icon>>(
        &self,
        icon: &P,
        size: i32,
        scale: i32,
        flags: IconLookupFlags,
    ) -> Option<IconInfo>;

    fn lookup_icon(&self, icon_name: &str, size: i32, flags: IconLookupFlags) -> Option<IconInfo>;

    fn lookup_icon_for_scale(
        &self,
        icon_name: &str,
        size: i32,
        scale: i32,
        flags: IconLookupFlags,
    ) -> Option<IconInfo>;

    fn prepend_search_path<P: AsRef<std::path::Path>>(&self, path: P);

    fn rescan_if_needed(&self) -> bool;

    fn set_custom_theme(&self, theme_name: Option<&str>);

    fn set_screen(&self, screen: &gdk::Screen);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<IconTheme>> IconThemeExt for O {
    fn add_resource_path(&self, path: &str) {
        unsafe {
            gtk_sys::gtk_icon_theme_add_resource_path(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
            );
        }
    }

    fn append_search_path<P: AsRef<std::path::Path>>(&self, path: P) {
        unsafe {
            gtk_sys::gtk_icon_theme_append_search_path(
                self.as_ref().to_glib_none().0,
                path.as_ref().to_glib_none().0,
            );
        }
    }

    fn get_example_icon_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_icon_theme_get_example_icon_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_icon(&self, icon_name: &str) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_icon_theme_has_icon(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
            ))
        }
    }

    fn list_contexts(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gtk_sys::gtk_icon_theme_list_contexts(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn list_icons(&self, context: Option<&str>) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gtk_sys::gtk_icon_theme_list_icons(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
            ))
        }
    }

    fn load_icon(
        &self,
        icon_name: &str,
        size: i32,
        flags: IconLookupFlags,
    ) -> Result<Option<gdk_pixbuf::Pixbuf>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_icon_theme_load_icon(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
                size,
                flags.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn load_icon_for_scale(
        &self,
        icon_name: &str,
        size: i32,
        scale: i32,
        flags: IconLookupFlags,
    ) -> Result<Option<gdk_pixbuf::Pixbuf>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_icon_theme_load_icon_for_scale(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
                size,
                scale,
                flags.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn load_surface<P: IsA<gdk::Window>>(
        &self,
        icon_name: &str,
        size: i32,
        scale: i32,
        for_window: Option<&P>,
        flags: IconLookupFlags,
    ) -> Result<Option<cairo::Surface>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_icon_theme_load_surface(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
                size,
                scale,
                for_window.map(|p| p.as_ref()).to_glib_none().0,
                flags.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn lookup_by_gicon<P: IsA<gio::Icon>>(
        &self,
        icon: &P,
        size: i32,
        flags: IconLookupFlags,
    ) -> Option<IconInfo> {
        unsafe {
            from_glib_full(gtk_sys::gtk_icon_theme_lookup_by_gicon(
                self.as_ref().to_glib_none().0,
                icon.as_ref().to_glib_none().0,
                size,
                flags.to_glib(),
            ))
        }
    }

    fn lookup_by_gicon_for_scale<P: IsA<gio::Icon>>(
        &self,
        icon: &P,
        size: i32,
        scale: i32,
        flags: IconLookupFlags,
    ) -> Option<IconInfo> {
        unsafe {
            from_glib_full(gtk_sys::gtk_icon_theme_lookup_by_gicon_for_scale(
                self.as_ref().to_glib_none().0,
                icon.as_ref().to_glib_none().0,
                size,
                scale,
                flags.to_glib(),
            ))
        }
    }

    fn lookup_icon(&self, icon_name: &str, size: i32, flags: IconLookupFlags) -> Option<IconInfo> {
        unsafe {
            from_glib_full(gtk_sys::gtk_icon_theme_lookup_icon(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
                size,
                flags.to_glib(),
            ))
        }
    }

    fn lookup_icon_for_scale(
        &self,
        icon_name: &str,
        size: i32,
        scale: i32,
        flags: IconLookupFlags,
    ) -> Option<IconInfo> {
        unsafe {
            from_glib_full(gtk_sys::gtk_icon_theme_lookup_icon_for_scale(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
                size,
                scale,
                flags.to_glib(),
            ))
        }
    }

    fn prepend_search_path<P: AsRef<std::path::Path>>(&self, path: P) {
        unsafe {
            gtk_sys::gtk_icon_theme_prepend_search_path(
                self.as_ref().to_glib_none().0,
                path.as_ref().to_glib_none().0,
            );
        }
    }

    fn rescan_if_needed(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_icon_theme_rescan_if_needed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_custom_theme(&self, theme_name: Option<&str>) {
        unsafe {
            gtk_sys::gtk_icon_theme_set_custom_theme(
                self.as_ref().to_glib_none().0,
                theme_name.to_glib_none().0,
            );
        }
    }

    fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            gtk_sys::gtk_icon_theme_set_screen(
                self.as_ref().to_glib_none().0,
                screen.to_glib_none().0,
            );
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkIconTheme,
            f: glib_sys::gpointer,
        ) where
            P: IsA<IconTheme>,
        {
            let f: &F = &*(f as *const F);
            f(&IconTheme::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for IconTheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IconTheme")
    }
}
