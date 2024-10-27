mod app;
mod connection;
mod menu;

use app::*;
use leptos::*;
use leptos_i18n::load_locales;
use thaw::MessageProvider;

load_locales!();

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <MessageProvider>
                <App/>
            </MessageProvider>
        }
    })
}
