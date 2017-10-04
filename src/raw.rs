use va_list::VaList;
use ::glue::*;

extern "C" {
    pub fn g_strdup_vprintf(format: *const gchar, args: VaList) -> *mut gchar;
}
