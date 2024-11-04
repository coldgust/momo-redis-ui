use crate::global_state::{GlobalState, TabProp};
use crate::panel::panel::Panel;
use crate::sider::sider::Sider;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn App() -> impl IntoView {
    provide_context(GlobalState::default());

    let global_state = use_context::<GlobalState>().unwrap();
    let uuid = Uuid::new_v4().to_string();
    global_state.tabs.update(|t| t.push(TabProp {
        name: "test".into(),
        id: uuid.clone(),
    }));
    global_state.selected_tab.set(uuid);

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
