use crate::obj_ref_to_ptr;
use objc::{
    msg_send,
    runtime::{Object, BOOL, NO},
    sel, sel_impl,
};
use objc_derive::{objc_impl, selector_impl};
use tao_foundation::{id, NSString};
use tao_uikit::{UIColor, UILabel, UIView, UIViewController};

pub(crate) struct MainViewController {}

impl MainViewController {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[objc_impl(UIViewController)]
impl MainViewController {
    #[selector_impl("loadView")]
    fn load_view(&self, this: &Object) {
        unsafe {
            let view = UIView::alloc();
            let view: id = msg_send![view, init];
            let view = UIView(view);
            view.set_translates_autoresizing_mask_into_constraints(NO);

            let id = obj_ref_to_ptr(this);
            let ui_view_controller = UIViewController(id);
            ui_view_controller.set_view(view);

            println!("MainViewController load view");
        }
    }

    #[selector_impl("viewDidLoad")]
    fn view_did_load(&self, this: &Object) {
        println!("MainViewController did load");
        unsafe {
            let id = obj_ref_to_ptr(this);
            let ui_view_controller = UIViewController(id);

            ui_view_controller
                .view()
                .set_background_color(UIColor::white_color());

            let label = {
                let label = UILabel::alloc();
                let label: UILabel = msg_send!(label, init);

                label.set_text(NSString::from_str("Hello From Rust"));

                label
            };
            let label_ui_view = UIView(label.0);
            label_ui_view.set_translates_autoresizing_mask_into_constraints(NO);

            ui_view_controller.view().add_subview(label_ui_view);
        }
    }

    #[selector_impl("viewDidAppear:")]
    fn view_did_appear(&self, _this: &Object, _animated: BOOL) {
        println!("MainViewController view did appear");
    }
}
