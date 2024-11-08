use lv2_raw::*; type LV2_Feature = LV2Feature;
/* automatically generated by rust-bindgen 0.70.1 */

#[doc = "UI controller.\n\nThis is an opaque pointer passed by the user which is passed to the various\nUI control functions (e.g. SuilPortWriteFunc).  It is typically used to pass\na pointer to some controller object the host uses to communicate with\nplugins."]
pub type SuilController = *mut ::std::os::raw::c_void;
#[doc = " Function to write/send a value to a port"]
pub type SuilPortWriteFunc = ::std::option::Option<
    unsafe extern "C" fn(
        controller: SuilController,
        port_index: u32,
        buffer_size: u32,
        protocol: u32,
        buffer: *const ::std::os::raw::c_void,
    ),
>;
#[doc = " Function to return the index for a port by symbol"]
pub type SuilPortIndexFunc = ::std::option::Option<
    unsafe extern "C" fn(
        controller: SuilController,
        port_symbol: *const ::std::os::raw::c_char,
    ) -> u32,
>;
#[doc = " Function to subscribe to notifications for a port"]
pub type SuilPortSubscribeFunc = ::std::option::Option<
    unsafe extern "C" fn(
        controller: SuilController,
        port_index: u32,
        protocol: u32,
        features: *const *const LV2_Feature,
    ) -> u32,
>;
#[doc = " Function to unsubscribe from notifications for a port"]
pub type SuilPortUnsubscribeFunc = ::std::option::Option<
    unsafe extern "C" fn(
        controller: SuilController,
        port_index: u32,
        protocol: u32,
        features: *const *const LV2_Feature,
    ) -> u32,
>;
#[doc = " Function called when a control is grabbed or released"]
pub type SuilTouchFunc = ::std::option::Option<
    unsafe extern "C" fn(controller: SuilController, port_index: u32, grabbed: bool),
>;
pub const SuilArg_SUIL_ARG_NONE: SuilArg = 0;
#[doc = " Initialization argument"]
pub type SuilArg = ::std::os::raw::c_uint;
extern "C" {
    #[doc = "Initialize suil.\n\nThis function should be called as early as possible, before any other GUI\ntoolkit functions.  The variable argument list is a sequence of SuilArg keys\nand corresponding value pairs for passing any necessary platform-specific\ninformation.  It must be terminated with SUIL_ARG_NONE."]
    pub fn suil_init(
        argc: *mut ::std::os::raw::c_int,
        argv: *mut *mut *mut ::std::os::raw::c_char,
        key: SuilArg,
        ...
    );
}
extern "C" {
    #[doc = "Check if suil can wrap a UI type.\n\n@param host_type_uri The URI of the desired widget type of the host,\ncorresponding to the `type_uri` parameter of suil_instance_new().\n\n@param ui_type_uri The URI of the UI widget type.\n\n@return 0 if wrapping is unsupported, otherwise the quality of the wrapping\nwhere 1 is the highest quality (direct native embedding with no wrapping)\nand increasing values are of a progressively lower quality and/or stability."]
    pub fn suil_ui_supported(
        host_type_uri: *const ::std::os::raw::c_char,
        ui_type_uri: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_uint;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SuilHostImpl {
    _unused: [u8; 0],
}
#[doc = "UI host descriptor.\n\nThis contains the various functions that a plugin UI may use to communicate\nwith the plugin.  It is passed to suil_instance_new() to provide these\nfunctions to the UI."]
pub type SuilHost = SuilHostImpl;
extern "C" {
    #[doc = "Create a new UI host descriptor.\n\n@param write_func Function to send a value to a plugin port.\n@param index_func Function to get the index for a port by symbol.\n@param subscribe_func Function to subscribe to port updates.\n@param unsubscribe_func Function to unsubscribe from port updates."]
    pub fn suil_host_new(
        write_func: SuilPortWriteFunc,
        index_func: SuilPortIndexFunc,
        subscribe_func: SuilPortSubscribeFunc,
        unsubscribe_func: SuilPortUnsubscribeFunc,
    ) -> *mut SuilHost;
}
extern "C" {
    #[doc = "Set a touch function for a host descriptor.\n\nNote this function will only be called if the UI supports it."]
    pub fn suil_host_set_touch_func(host: *mut SuilHost, touch_func: SuilTouchFunc);
}
extern "C" {
    #[doc = "Free `host`."]
    pub fn suil_host_free(host: *mut SuilHost);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SuilInstanceImpl {
    _unused: [u8; 0],
}
#[doc = " An instance of an LV2 plugin UI"]
pub type SuilInstance = SuilInstanceImpl;
#[doc = " Opaque pointer to a UI handle"]
pub type SuilHandle = *mut ::std::os::raw::c_void;
#[doc = " Opaque pointer to a UI widget"]
pub type SuilWidget = *mut ::std::os::raw::c_void;
extern "C" {
    #[doc = "Instantiate a UI for an LV2 plugin.\n\nThis funcion may load a suil module to adapt the UI to the desired toolkit.\nSuil is configured at compile time to load modules from the appropriate\nplace, but this can be changed at run-time via the environment variable\nSUIL_MODULE_DIR.  This makes it possible to bundle suil with an application.\n\nNote that some situations (Gtk in Qt, Windows in Gtk) require a parent\ncontainer to be passed as a feature with URI LV2_UI__parent\n(http://lv2plug.in/ns/extensions/ui#ui) in order to work correctly.  The\ndata must point to a single child container of the host widget set.\n\n@param host Host descriptor.\n@param controller Opaque host controller pointer.\n@param container_type_uri URI of the desired host container widget type.\n@param plugin_uri URI of the plugin to instantiate this UI for.\n@param ui_uri URI of the specifically desired UI.\n@param ui_type_uri URI of the actual UI widget type.\n@param ui_bundle_path Path of the UI bundle.\n@param ui_binary_path Path of the UI binary.\n@param features NULL-terminated array of supported features, or NULL.\n@return A new UI instance, or NULL if instantiation failed."]
    pub fn suil_instance_new(
        host: *mut SuilHost,
        controller: SuilController,
        container_type_uri: *const ::std::os::raw::c_char,
        plugin_uri: *const ::std::os::raw::c_char,
        ui_uri: *const ::std::os::raw::c_char,
        ui_type_uri: *const ::std::os::raw::c_char,
        ui_bundle_path: *const ::std::os::raw::c_char,
        ui_binary_path: *const ::std::os::raw::c_char,
        features: *const *const LV2_Feature,
    ) -> *mut SuilInstance;
}
extern "C" {
    #[doc = "Free a plugin UI instance.\n\nThe caller must ensure all references to the UI have been dropped before\ncalling this function (e.g. it has been removed from its parent)."]
    pub fn suil_instance_free(instance: *mut SuilInstance);
}
extern "C" {
    #[doc = "Get the handle for a UI instance.\n\nReturns the handle to the UI instance.  The returned handle has opaque type\nto insulate the Suil API from LV2 extensions, but in pactice it is currently\nof type `LV2UI_Handle`.  This should not normally be needed.\n\nThe returned handle is shared and must not be deleted."]
    pub fn suil_instance_get_handle(instance: *mut SuilInstance) -> SuilHandle;
}
extern "C" {
    #[doc = "Get the widget for a UI instance.\n\nReturns an opaque pointer to a widget, the type of which matches the\n`container_type_uri` parameter of suil_instance_new().  Note this may be a\nwrapper widget created by Suil, and not necessarily the widget directly\nimplemented by the UI."]
    pub fn suil_instance_get_widget(instance: *mut SuilInstance) -> SuilWidget;
}
extern "C" {
    #[doc = "Notify the UI about a change in a plugin port.\n\nThis function can be used to notify the UI about any port change, but in the\nsimplest case is used to set the value of lv2:ControlPort ports.  For\nsimplicity, this is a special case where `format` is 0, `buffer_size` is 4,\nand `buffer` should point to a single float.\n\nThe `buffer` must be valid only for the duration of this call, the UI must\nnot keep a reference to it.\n\n@param instance UI instance.\n@param port_index Index of the port which has changed.\n@param buffer_size Size of `buffer` in bytes.\n@param format Format of `buffer` (mapped URI, or 0 for float).\n@param buffer Change data, e.g. the new port value."]
    pub fn suil_instance_port_event(
        instance: *mut SuilInstance,
        port_index: u32,
        buffer_size: u32,
        format: u32,
        buffer: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " Return a data structure defined by some LV2 extension URI"]
    pub fn suil_instance_extension_data(
        instance: *mut SuilInstance,
        uri: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_void;
}
