mod app;
mod connection;
mod menu;

use app::*;
use leptos::prelude::*;
use leptos_i18n::load_locales;
use thaw::{ConfigProvider, ToasterProvider};

load_locales!();

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <ConfigProvider>
                <ToasterProvider>
                    <App/>
                </ToasterProvider>
            </ConfigProvider>
        }
    })
}
