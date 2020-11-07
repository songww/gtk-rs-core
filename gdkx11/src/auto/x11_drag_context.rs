// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_x11_sys;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct X11DragContext(Object<gdk_x11_sys::GdkX11DragContext, gdk_x11_sys::GdkX11DragContextClass>) @extends gdk::DragContext;

    match fn {
        get_type => || gdk_x11_sys::gdk_x11_drag_context_get_type(),
    }
}

impl X11DragContext {}

impl fmt::Display for X11DragContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11DragContext")
    }
}
