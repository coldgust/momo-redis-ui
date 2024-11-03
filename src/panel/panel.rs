use leptos::prelude::*;
use thaw::*;
use crate::global_state::GlobalState;

#[component]
pub fn Panel() -> impl IntoView {
    let global_state = use_context::<GlobalState>().unwrap();
    let selected_value = global_state.selected_tab;
    let tabs = global_state.tabs;

    view! {
        <TabList selected_value>
            <Tab value="">test</Tab>
        </TabList>
    }
}