use leptos::prelude::*;
use thaw::{Button, ButtonAppearance, Flex, FlexJustify};
use crate::i18n::{t, use_i18n};
use crate::new_connection::new_connection::NewConnection;
use crate::settings::settings::Settings;

#[component]
pub fn Sider() -> impl IntoView {
    let i18n = use_i18n();
    let (style_sheet, class_names) = turf::style_sheet_values!("src/sider/sider.scss");

    let new_conn_open = RwSignal::new(false);
    let settings_open = RwSignal::new(false);

    view! {
        <style>{style_sheet}</style>
        <Flex justify=FlexJustify::SpaceBetween>
            <Button appearance=ButtonAppearance::Primary class=class_names.button
                on:click=move |_| new_conn_open.set(true)>{t!(i18n, new_connection)}</Button>
            <Button class=class_names.button
                on:click=move |_| settings_open.set(true)>{t!(i18n, settings)}</Button>
        </Flex>
        <Settings open=settings_open />
        <NewConnection open=new_conn_open />
    }
}