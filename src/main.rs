mod app;
mod sider;
mod settings;
mod new_connection;
mod panel;
mod global_state;

use app::*;
use leptos::prelude::*;
use leptos_i18n::load_locales;
use thaw::{ConfigProvider, ToasterProvider};
use crate::i18n::I18nContextProvider;

load_locales!();

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <I18nContextProvider>
                <ConfigProvider>
                    <ToasterProvider>
                        <App/>
                    </ToasterProvider>
                </ConfigProvider>
            </I18nContextProvider>
        }
    })
}
