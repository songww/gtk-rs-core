// This file was generated by gir (7dd2bcd) from gir-files (11e0e6d)
// DO NOT EDIT

use TextBuffer;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct TextMark(Object<ffi::GtkTextMark>);

    match fn {
        get_type => || ffi::gtk_text_mark_get_type(),
    }
}

impl TextMark {
    pub fn new(name: Option<&str>, left_gravity: bool) -> TextMark {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_text_mark_new(name.to_glib_none().0, left_gravity.to_glib()))
        }
    }

    pub fn get_buffer(&self) -> Option<TextBuffer> {
        unsafe {
            from_glib_none(ffi::gtk_text_mark_get_buffer(self.to_glib_none().0))
        }
    }

    pub fn get_deleted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_deleted(self.to_glib_none().0))
        }
    }

    pub fn get_left_gravity(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_left_gravity(self.to_glib_none().0))
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_text_mark_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_visible(self.to_glib_none().0))
        }
    }

    pub fn set_visible(&self, setting: bool) {
        unsafe {
            ffi::gtk_text_mark_set_visible(self.to_glib_none().0, setting.to_glib());
        }
    }
}
