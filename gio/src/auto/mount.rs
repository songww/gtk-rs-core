// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::Drive;
use crate::File;
use crate::Icon;
use crate::MountMountFlags;
use crate::MountOperation;
use crate::MountUnmountFlags;
use crate::Volume;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GMount")]
    pub struct Mount(Interface<ffi::GMount, ffi::GMountIface>);

    match fn {
        type_ => || ffi::g_mount_get_type(),
    }
}

impl Mount {
    pub const NONE: Option<&'static Mount> = None;
}

pub trait MountExt: 'static {
    #[doc(alias = "g_mount_can_eject")]
    fn can_eject(&self) -> bool;

    #[doc(alias = "g_mount_can_unmount")]
    fn can_unmount(&self) -> bool;

    #[doc(alias = "g_mount_eject_with_operation")]
    fn eject_with_operation<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn eject_with_operation_future(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_mount_get_default_location")]
    #[doc(alias = "get_default_location")]
    fn default_location(&self) -> File;

    #[doc(alias = "g_mount_get_drive")]
    #[doc(alias = "get_drive")]
    fn drive(&self) -> Option<Drive>;

    #[doc(alias = "g_mount_get_icon")]
    #[doc(alias = "get_icon")]
    fn icon(&self) -> Icon;

    #[doc(alias = "g_mount_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> glib::GString;

    #[doc(alias = "g_mount_get_root")]
    #[doc(alias = "get_root")]
    fn root(&self) -> File;

    #[doc(alias = "g_mount_get_sort_key")]
    #[doc(alias = "get_sort_key")]
    fn sort_key(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mount_get_symbolic_icon")]
    #[doc(alias = "get_symbolic_icon")]
    fn symbolic_icon(&self) -> Icon;

    #[doc(alias = "g_mount_get_uuid")]
    #[doc(alias = "get_uuid")]
    fn uuid(&self) -> Option<glib::GString>;

    #[doc(alias = "g_mount_get_volume")]
    #[doc(alias = "get_volume")]
    fn volume(&self) -> Option<Volume>;

    #[doc(alias = "g_mount_guess_content_type")]
    fn guess_content_type<P: FnOnce(Result<Vec<glib::GString>, glib::Error>) + 'static>(
        &self,
        force_rescan: bool,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn guess_content_type_future(
        &self,
        force_rescan: bool,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<Vec<glib::GString>, glib::Error>> + 'static>,
    >;

    #[doc(alias = "g_mount_guess_content_type_sync")]
    fn guess_content_type_sync(
        &self,
        force_rescan: bool,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Vec<glib::GString>, glib::Error>;

    #[doc(alias = "g_mount_is_shadowed")]
    fn is_shadowed(&self) -> bool;

    #[doc(alias = "g_mount_remount")]
    fn remount<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn remount_future(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_mount_shadow")]
    fn shadow(&self);

    #[doc(alias = "g_mount_unmount_with_operation")]
    fn unmount_with_operation<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn unmount_with_operation_future(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_mount_unshadow")]
    fn unshadow(&self);

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "pre-unmount")]
    fn connect_pre_unmount<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "unmounted")]
    fn connect_unmounted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Mount>> MountExt for O {
    fn can_eject(&self) -> bool {
        unsafe { from_glib(ffi::g_mount_can_eject(self.as_ref().to_glib_none().0)) }
    }

    fn can_unmount(&self) -> bool {
        unsafe { from_glib(ffi::g_mount_can_unmount(self.as_ref().to_glib_none().0)) }
    }

    fn eject_with_operation<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn eject_with_operation_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::g_mount_eject_with_operation_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = eject_with_operation_trampoline::<P>;
        unsafe {
            ffi::g_mount_eject_with_operation(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn eject_with_operation_future(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.eject_with_operation(
                    flags,
                    mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    fn default_location(&self) -> File {
        unsafe {
            from_glib_full(ffi::g_mount_get_default_location(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn drive(&self) -> Option<Drive> {
        unsafe { from_glib_full(ffi::g_mount_get_drive(self.as_ref().to_glib_none().0)) }
    }

    fn icon(&self) -> Icon {
        unsafe { from_glib_full(ffi::g_mount_get_icon(self.as_ref().to_glib_none().0)) }
    }

    fn name(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::g_mount_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn root(&self) -> File {
        unsafe { from_glib_full(ffi::g_mount_get_root(self.as_ref().to_glib_none().0)) }
    }

    fn sort_key(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_mount_get_sort_key(self.as_ref().to_glib_none().0)) }
    }

    fn symbolic_icon(&self) -> Icon {
        unsafe {
            from_glib_full(ffi::g_mount_get_symbolic_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn uuid(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::g_mount_get_uuid(self.as_ref().to_glib_none().0)) }
    }

    fn volume(&self) -> Option<Volume> {
        unsafe { from_glib_full(ffi::g_mount_get_volume(self.as_ref().to_glib_none().0)) }
    }

    fn guess_content_type<P: FnOnce(Result<Vec<glib::GString>, glib::Error>) + 'static>(
        &self,
        force_rescan: bool,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn guess_content_type_trampoline<
            P: FnOnce(Result<Vec<glib::GString>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_mount_guess_content_type_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = guess_content_type_trampoline::<P>;
        unsafe {
            ffi::g_mount_guess_content_type(
                self.as_ref().to_glib_none().0,
                force_rescan.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn guess_content_type_future(
        &self,
        force_rescan: bool,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<Vec<glib::GString>, glib::Error>> + 'static>,
    > {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.guess_content_type(force_rescan, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    fn guess_content_type_sync(
        &self,
        force_rescan: bool,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Vec<glib::GString>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mount_guess_content_type_sync(
                self.as_ref().to_glib_none().0,
                force_rescan.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn is_shadowed(&self) -> bool {
        unsafe { from_glib(ffi::g_mount_is_shadowed(self.as_ref().to_glib_none().0)) }
    }

    fn remount<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn remount_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_mount_remount_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = remount_trampoline::<P>;
        unsafe {
            ffi::g_mount_remount(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn remount_future(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.remount(
                    flags,
                    mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    fn shadow(&self) {
        unsafe {
            ffi::g_mount_shadow(self.as_ref().to_glib_none().0);
        }
    }

    fn unmount_with_operation<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&impl IsA<MountOperation>>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn unmount_with_operation_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_mount_unmount_with_operation_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = unmount_with_operation_trampoline::<P>;
        unsafe {
            ffi::g_mount_unmount_with_operation(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn unmount_with_operation_future(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&(impl IsA<MountOperation> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.unmount_with_operation(
                    flags,
                    mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    fn unshadow(&self) {
        unsafe {
            ffi::g_mount_unshadow(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P: IsA<Mount>, F: Fn(&P) + 'static>(
            this: *mut ffi::GMount,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Mount::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_pre_unmount<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn pre_unmount_trampoline<P: IsA<Mount>, F: Fn(&P) + 'static>(
            this: *mut ffi::GMount,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Mount::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pre-unmount\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    pre_unmount_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_unmounted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn unmounted_trampoline<P: IsA<Mount>, F: Fn(&P) + 'static>(
            this: *mut ffi::GMount,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Mount::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unmounted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    unmounted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Mount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Mount")
    }
}
