use crate::global_state::GlobalState;
use crate::panel::panel::Panel;
use crate::sider::sider::Sider;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    provide_context(GlobalState::default());

    let (style_sheet, class_names) = turf::style_sheet_values!("src/app.scss");

    view! {
        <style>{style_sheet}</style>
        <div>
            <div class=class_names.sider>
                <div class=class_names.resize_bar></div>
                <div class=class_names.resize_line></div>
                <div class=class_names.resize_content>
                    <Sider />
                </div>
            </div>
            <div class=class_names.content>
                <Panel />
            </div>
        </div>
    }
}
