#include <dlfcn.h>
#include <stdarg.h>
#include "glue.h"

GtkWidget * gtk_message_dialog_new(GtkWindow *parent, GtkDialogFlags flags, GtkMessageType type, GtkButtonsType buttons, const gchar *format, ...) {
    void *orig = dlsym(RTLD_NEXT, "gtk_message_dialog_new");

    va_list args;
    va_start(args, format);

    GtkWidget *ret = rust_gtk_message_dialog_new(orig, parent, flags, type, buttons, format, args);

    va_end(args);

    return ret;
}
