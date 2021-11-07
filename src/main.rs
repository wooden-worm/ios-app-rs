extern crate block;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr::null_mut;
use std::sync::Arc;

use main_vc::MainViewController;
use objc::runtime::{Object, BOOL, YES};
use objc::{msg_send, sel, sel_impl};
use objc_derive::{objc_impl, selector_impl, selector_init};
use tao_foundation::{id, NSClassFromString, NSString};
use tao_uikit::{UISceneConfiguration, UIViewController, UIWindow};

mod main_vc;

extern "C" {
    /// Required for iOS applications to initialize.
    fn UIApplicationMain(
        argc: c_int,
        argv: *const *const c_char,
        principal_class_name: id,
        delegate_class_name: id,
    );
}

fn main() {
    let args = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>();

    let c_args = args
        .iter()
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();

    let delegate_class_name = NSString::from_str(MyApplicationDelegate::objc_class_name());

    MyApplicationDelegate::register_rust_class();
    WindowSceneDelegate::register_rust_class();

    unsafe {
        UIApplicationMain(
            c_args.len() as c_int,
            c_args.as_ptr(),
            null_mut(),
            delegate_class_name.0,
        );
    }
}

struct MyApplicationDelegate;

#[objc_impl(NSObject)]
impl MyApplicationDelegate {
    #[selector_init]
    fn init() -> Arc<Self> {
        Arc::new(Self)
    }

    #[selector_impl("application:didFinishLaunchingWithOptions:")]
    fn did_finish_launching(&self, _this: &Object, _application: id, _launch_options: id) -> BOOL {
        YES
    }

    #[selector_impl("application:configurationForConnectingSceneSession:options:")]
    fn config_scene(
        &self,
        _this: &Object,
        _application: id,
        _connecting_scene_session: id,
        _options: id,
    ) -> id {
        unsafe {
            let ret = UISceneConfiguration::configuration_with_name_session_role(
                NSString::from_str("Default Config"),
                NSString::from_str("Application"),
            );

            let delegate_class =
                NSClassFromString(NSString::from_str(WindowSceneDelegate::objc_class_name()));

            ret.set_delegate_class(delegate_class);
            ret.0
        }
    }
}

struct WindowSceneDelegate {}

#[objc_impl(NSObject, UIWindowSceneDelegate)]
impl WindowSceneDelegate {
    #[selector_init]
    fn init() -> Arc<Self> {
        Arc::new(Self {})
    }

    #[selector_impl("scene:willConnectToSession:options:")]
    fn will_connect(&self, _this: &Object, scene: id, _session: id, _connection_options: id) {
        let window = UIWindow::alloc();
        let window = window.init_with_window_scene(scene);

        let root_vc = Arc::new(MainViewController::new());
        let obj_vc = root_vc.init_objc_proxy_obj();

        window.set_root_view_controller(UIViewController(obj_vc));
        window.make_key_and_visible();
    }
}

fn obj_ref_to_ptr(this: &Object) -> id {
    let raw_ptr = this as *const Object;
    let ptr_value = raw_ptr as usize;
    let ret = ptr_value as *mut Object;
    ret
}
