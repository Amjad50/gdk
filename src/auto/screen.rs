// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Display;
use Rectangle;
use Visual;
use Window;
use cairo;
use gdk_sys;
use glib::GString;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Screen(Object<gdk_sys::GdkScreen, ScreenClass>);

    match fn {
        get_type => || gdk_sys::gdk_screen_get_type(),
    }
}

impl Screen {
    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_active_window(&self) -> Option<Window> {
        unsafe {
            from_glib_full(gdk_sys::gdk_screen_get_active_window(self.to_glib_none().0))
        }
    }

    pub fn get_display(&self) -> Display {
        unsafe {
            from_glib_none(gdk_sys::gdk_screen_get_display(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_height(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_screen_get_height(self.to_glib_none().0)
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_height_mm(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_screen_get_height_mm(self.to_glib_none().0)
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_monitor_at_point(&self, x: i32, y: i32) -> i32 {
        unsafe {
            gdk_sys::gdk_screen_get_monitor_at_point(self.to_glib_none().0, x, y)
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_monitor_at_window<P: IsA<Window>>(&self, window: &P) -> i32 {
        unsafe {
            gdk_sys::gdk_screen_get_monitor_at_window(self.to_glib_none().0, window.as_ref().to_glib_none().0)
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_monitor_geometry(&self, monitor_num: i32) -> Rectangle {
        unsafe {
            let mut dest = Rectangle::uninitialized();
            gdk_sys::gdk_screen_get_monitor_geometry(self.to_glib_none().0, monitor_num, dest.to_glib_none_mut().0);
            dest
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_monitor_height_mm(&self, monitor_num: i32) -> i32 {
        unsafe {
            gdk_sys::gdk_screen_get_monitor_height_mm(self.to_glib_none().0, monitor_num)
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_monitor_plug_name(&self, monitor_num: i32) -> Option<GString> {
        unsafe {
            from_glib_full(gdk_sys::gdk_screen_get_monitor_plug_name(self.to_glib_none().0, monitor_num))
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_monitor_scale_factor(&self, monitor_num: i32) -> i32 {
        unsafe {
            gdk_sys::gdk_screen_get_monitor_scale_factor(self.to_glib_none().0, monitor_num)
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_monitor_width_mm(&self, monitor_num: i32) -> i32 {
        unsafe {
            gdk_sys::gdk_screen_get_monitor_width_mm(self.to_glib_none().0, monitor_num)
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_monitor_workarea(&self, monitor_num: i32) -> Rectangle {
        unsafe {
            let mut dest = Rectangle::uninitialized();
            gdk_sys::gdk_screen_get_monitor_workarea(self.to_glib_none().0, monitor_num, dest.to_glib_none_mut().0);
            dest
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_n_monitors(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_screen_get_n_monitors(self.to_glib_none().0)
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_number(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_screen_get_number(self.to_glib_none().0)
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_primary_monitor(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_screen_get_primary_monitor(self.to_glib_none().0)
        }
    }

    pub fn get_resolution(&self) -> f64 {
        unsafe {
            gdk_sys::gdk_screen_get_resolution(self.to_glib_none().0)
        }
    }

    pub fn get_rgba_visual(&self) -> Option<Visual> {
        unsafe {
            from_glib_none(gdk_sys::gdk_screen_get_rgba_visual(self.to_glib_none().0))
        }
    }

    pub fn get_root_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(gdk_sys::gdk_screen_get_root_window(self.to_glib_none().0))
        }
    }

    pub fn get_system_visual(&self) -> Option<Visual> {
        unsafe {
            from_glib_none(gdk_sys::gdk_screen_get_system_visual(self.to_glib_none().0))
        }
    }

    pub fn get_toplevel_windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gdk_sys::gdk_screen_get_toplevel_windows(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_width(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_screen_get_width(self.to_glib_none().0)
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn get_width_mm(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_screen_get_width_mm(self.to_glib_none().0)
        }
    }

    pub fn get_window_stack(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gdk_sys::gdk_screen_get_window_stack(self.to_glib_none().0))
        }
    }

    pub fn is_composited(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_screen_is_composited(self.to_glib_none().0))
        }
    }

    pub fn list_visuals(&self) -> Vec<Visual> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gdk_sys::gdk_screen_list_visuals(self.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn make_display_name(&self) -> GString {
        unsafe {
            from_glib_full(gdk_sys::gdk_screen_make_display_name(self.to_glib_none().0))
        }
    }

    pub fn set_font_options(&self, options: Option<&cairo::FontOptions>) {
        unsafe {
            gdk_sys::gdk_screen_set_font_options(self.to_glib_none().0, options.to_glib_none().0);
        }
    }

    pub fn set_resolution(&self, dpi: f64) {
        unsafe {
            gdk_sys::gdk_screen_set_resolution(self.to_glib_none().0, dpi);
        }
    }

    //pub fn get_property_font_options(&self) -> /*Unimplemented*/Fundamental: Pointer {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"font-options\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().unwrap()
    //    }
    //}

    pub fn get_default() -> Option<Screen> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gdk_sys::gdk_screen_get_default())
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn height() -> i32 {
        assert_initialized_main_thread!();
        unsafe {
            gdk_sys::gdk_screen_height()
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn height_mm() -> i32 {
        assert_initialized_main_thread!();
        unsafe {
            gdk_sys::gdk_screen_height_mm()
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn width() -> i32 {
        assert_initialized_main_thread!();
        unsafe {
            gdk_sys::gdk_screen_width()
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    pub fn width_mm() -> i32 {
        assert_initialized_main_thread!();
        unsafe {
            gdk_sys::gdk_screen_width_mm()
        }
    }

    pub fn connect_composited_changed<F: Fn(&Screen) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn composited_changed_trampoline<F: Fn(&Screen) + 'static>(this: *mut gdk_sys::GdkScreen, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"composited-changed\0".as_ptr() as *const _,
                Some(transmute(composited_changed_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_monitors_changed<F: Fn(&Screen) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn monitors_changed_trampoline<F: Fn(&Screen) + 'static>(this: *mut gdk_sys::GdkScreen, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"monitors-changed\0".as_ptr() as *const _,
                Some(transmute(monitors_changed_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_size_changed<F: Fn(&Screen) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn size_changed_trampoline<F: Fn(&Screen) + 'static>(this: *mut gdk_sys::GdkScreen, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"size-changed\0".as_ptr() as *const _,
                Some(transmute(size_changed_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_font_options_notify<F: Fn(&Screen) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_options_trampoline<F: Fn(&Screen) + 'static>(this: *mut gdk_sys::GdkScreen, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::font-options\0".as_ptr() as *const _,
                Some(transmute(notify_font_options_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_resolution_notify<F: Fn(&Screen) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_trampoline<F: Fn(&Screen) + 'static>(this: *mut gdk_sys::GdkScreen, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::resolution\0".as_ptr() as *const _,
                Some(transmute(notify_resolution_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Screen")
    }
}
