macro_rules! shim {
    () => ();
    { pub unsafe extern fn $name:ident($orig:ident, $($arg:ident : $arg_t:ty),*) $(-> $ret:ty)* $body:block $($rest:tt)* } => {
        #[no_mangle]
        pub unsafe extern fn $name($($arg: $arg_t),*) $(-> $ret)* {
            let $orig = {
                lazy_static! {
                    static ref ORIG: unsafe extern fn($($arg: $arg_t),*) $(-> $ret)* = unsafe {
                        let orig = ::libc::dlsym(::libc::RTLD_NEXT, concat!(stringify!($name), "\0").as_ptr() as _);

                        if orig.is_null() {
                            eprintln!(concat!("Unable to load `", stringify!($name), "` from shared library"));
                            ::std::process::abort();
                        }

                        ::std::mem::transmute(orig)
                    };
                }
                *ORIG
            };

            $body
        }

        shim!($($rest)*);
    }
}
