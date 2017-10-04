#pragma once

#include <stdarg.h>

typedef char gchar;
typedef int gint;
typedef void* gpointer;
typedef unsigned long gulong;


typedef struct GtkWindow GtkWindow;
typedef struct GtkWidget GtkWidget;
typedef struct GtkMenuShell GtkMenuShell;
typedef struct GtkMenuItem GtkMenuItem;
typedef struct GClosure GClosure;

typedef void (*GCallback) (void);
typedef void (*GClosureNotify) (gpointer data, GClosure *closure);

typedef enum GtkDialogFlags {
    GTK_DIALOG_MODAL,
    GTK_DIALOG_DESTROY_WITH_PARENT,
    GTK_DIALOG_NO_SEPARATOR,
} GtkDialogFlags;

typedef enum GtkMessageType {
    GTK_MESSAGE_INFO,
    GTK_MESSAGE_WARNING,
    GTK_MESSAGE_QUESTION,
    GTK_MESSAGE_ERROR,
    GTK_MESSAGE_OTHER,
} GtkMessageType ;

typedef enum GtkButtonsType {
    GTK_BUTTONS_NONE,
    GTK_BUTTONS_OK,
    GTK_BUTTONS_CLOSE,
    GTK_BUTTONS_CANCEL,
    GTK_BUTTONS_YES_NO,
    GTK_BUTTONS_OK_CANCEL,
} GtkButtonsType;

typedef enum GConnectFlags {
    G_CONNECT_AFTER,
    G_CONNECT_SWAPPED,
} GConnectFlags;

GtkWidget * rust_gtk_message_dialog_new(void *orig, GtkWindow *parent, GtkDialogFlags flags, GtkMessageType type, GtkButtonsType buttons, const gchar *format, va_list args);

gulong g_signal_connect_data(
    gpointer       instance,
    const gchar   *detailed_signal,
    GCallback      c_handler,
    gpointer       data,
    GClosureNotify destroy_data,
    GConnectFlags  connect_flags
);

void gtk_widget_hide(GtkWidget *widget);
