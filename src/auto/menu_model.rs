// This file was generated by gir (c3b4020) from gir-files (71d73f0)
// DO NOT EDIT

use MenuAttributeIter;
use MenuLinkIter;
use Object;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct MenuModel(Object<ffi::GMenuModel>);

    match fn {
        get_type => || ffi::g_menu_model_get_type(),
    }
}

pub trait MenuModelExt {
    //fn get_item_attribute(&self, item_index: i32, attribute: &str, format_string: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool;

    fn get_item_attribute_value(&self, item_index: i32, attribute: &str, expected_type: Option<&glib::VariantTy>) -> Option<glib::Variant>;

    fn get_item_link(&self, item_index: i32, link: &str) -> Option<MenuModel>;

    fn get_n_items(&self) -> i32;

    fn is_mutable(&self) -> bool;

    fn items_changed(&self, position: i32, removed: i32, added: i32);

    fn iterate_item_attributes(&self, item_index: i32) -> Option<MenuAttributeIter>;

    fn iterate_item_links(&self, item_index: i32) -> Option<MenuLinkIter>;

    fn connect_items_changed<F: Fn(&Self, i32, i32, i32) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<MenuModel> + IsA<Object>> MenuModelExt for O {
    //fn get_item_attribute(&self, item_index: i32, attribute: &str, format_string: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi::g_menu_model_get_item_attribute() }
    //}

    fn get_item_attribute_value(&self, item_index: i32, attribute: &str, expected_type: Option<&glib::VariantTy>) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_menu_model_get_item_attribute_value(self.to_glib_none().0, item_index, attribute.to_glib_none().0, expected_type.to_glib_none().0))
        }
    }

    fn get_item_link(&self, item_index: i32, link: &str) -> Option<MenuModel> {
        unsafe {
            from_glib_full(ffi::g_menu_model_get_item_link(self.to_glib_none().0, item_index, link.to_glib_none().0))
        }
    }

    fn get_n_items(&self) -> i32 {
        unsafe {
            ffi::g_menu_model_get_n_items(self.to_glib_none().0)
        }
    }

    fn is_mutable(&self) -> bool {
        unsafe {
            from_glib(ffi::g_menu_model_is_mutable(self.to_glib_none().0))
        }
    }

    fn items_changed(&self, position: i32, removed: i32, added: i32) {
        unsafe {
            ffi::g_menu_model_items_changed(self.to_glib_none().0, position, removed, added);
        }
    }

    fn iterate_item_attributes(&self, item_index: i32) -> Option<MenuAttributeIter> {
        unsafe {
            from_glib_full(ffi::g_menu_model_iterate_item_attributes(self.to_glib_none().0, item_index))
        }
    }

    fn iterate_item_links(&self, item_index: i32) -> Option<MenuLinkIter> {
        unsafe {
            from_glib_full(ffi::g_menu_model_iterate_item_links(self.to_glib_none().0, item_index))
        }
    }

    fn connect_items_changed<F: Fn(&Self, i32, i32, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, i32, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "items-changed",
                transmute(items_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn items_changed_trampoline<T>(this: *mut ffi::GMenuModel, position: libc::c_int, removed: libc::c_int, added: libc::c_int, f: glib_ffi::gpointer)
where T: IsA<MenuModel> {
    callback_guard!();
    let f: &Box_<Fn(&T, i32, i32, i32) + 'static> = transmute(f);
    f(&MenuModel::from_glib_none(this).downcast_unchecked(), position, removed, added)
}
