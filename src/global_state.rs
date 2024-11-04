use leptos::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct GlobalState {
    pub tabs: RwSignal<Vec<TabProp>>,
    pub selected_tab: RwSignal<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TabProp {
    pub name: String,
    pub id: String,
}