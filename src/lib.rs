#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;
extern crate libc;
extern crate apply;
extern crate xdg;
extern crate va_list;
extern crate regex;
extern crate rand;

use std::ffi::{CString,CStr};
use std::ptr;
use std::mem;
use apply::Apply;
use va_list::VaList;
use libc::c_void;

#[macro_use] mod log;
#[macro_use] mod shim;
mod glue;
mod config;
mod raw;

use glue::*;
use libc::{c_int,c_char};

shim! {
    pub unsafe extern fn gtk_init(orig, argc: *mut c_int, argv: *mut *mut *mut c_char) {
        log!("Init!");

        if !argc.is_null() && *argc > 0 && !argv.is_null() && !(*argv).is_null() {
            let command = CStr::from_ptr(**argv);
            let command = command.to_string_lossy();

            config::init(&command);

            log!("arg0: {}", command);
        }

        orig(argc, argv)
    }

    pub unsafe extern fn gtk_window_set_title(orig, window: *mut GtkWindow, title: *const gchar) {
        match config::title_pattern() {
            Some(re) => {
                let title = CStr::from_ptr(title).to_string_lossy();
                let title = re.replace(&title, config::title_replacement());
                let title = title.as_bytes().apply(CString::new).unwrap();
                orig(window, title.as_ptr())
            },
            None => orig(window, title),
        };
    }

    pub unsafe extern fn gtk_menu_item_new_with_label(orig, label: *const gchar) -> *mut GtkWidget {
        let item = orig(label);
        let label = CStr::from_ptr(label).to_string_lossy();

        if config::is_hidden_menu_item(&label) {
            log!("XXX [{:p}] new menu item w/ label: {}", item, label);
            hide_widget_perpetually(item as _);
        }

        item
    }

    pub unsafe extern fn gtk_menu_item_set_label(orig, item: *mut GtkMenuItem, label: *const gchar) {
        orig(item, label);
        let label = CStr::from_ptr(label).to_string_lossy();

        if config::is_hidden_menu_item(&label) {
            log!("XXX [{:p}] menu item set label: {}", item, label);

            hide_widget_perpetually(item as _);
        }
    }
}

unsafe fn hide_widget_perpetually(widget: *mut GtkWidget) {
    unsafe extern fn cb(widget: *mut GtkWidget, _user_data: gpointer) {
        log!("XXX [{:p}] show signal", widget);
        glue::gtk_widget_hide(widget);
    }

    glue::gtk_widget_hide(widget);
    glue::g_signal_connect_data(
        widget as _,
        "show\0".as_ptr() as _,
        Some(mem::transmute(cb as usize)),
        ptr::null_mut(),
        None,
        GConnectFlags::G_CONNECT_AFTER
    );
}

#[no_mangle]
pub unsafe extern fn rust_gtk_message_dialog_new(orig: *const c_void, parent: *mut GtkWindow, flags: GtkDialogFlags, typ: GtkMessageType, buttons: GtkButtonsType, format: *const gchar, args: VaList) -> *mut GtkWidget {
    log!("Calling rust_gtk_message_dialog_new");

    let orig: extern fn(*mut GtkWindow, GtkDialogFlags,GtkMessageType, GtkButtonsType, *const gchar, ...) -> *mut GtkWidget = ::std::mem::transmute(orig);
    let msg = raw::g_strdup_vprintf(format, args);

    if config::is_supressed_dialog(&CStr::from_ptr(msg).to_string_lossy()) {
        return ptr::null_mut();
    }

    orig(parent, flags, typ, buttons, "%s\0".as_ptr() as *const i8, msg)
}
