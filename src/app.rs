use api::ConnInfo;
use leptos::ev::{Event, SubmitEvent};
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use thaw::{use_message, Layout, LayoutHeader, LayoutSider, MessageOptions, MessageVariant, MessageProvider};
use wasm_bindgen::prelude::*;
use api::ConnAddr::Standalone;
use api::error::Error;

#[wasm_bindgen]
extern "C" {
    // invoke without arguments
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Deserialize, Serialize)]
struct SetArgs<'a> {
    key: &'a str,
    val: &'a str,
    info: ConnInfo,
}

#[derive(Deserialize, Serialize)]
struct GetArgs<'a> {
    key: &'a str,
    info: ConnInfo,
}

#[component]
pub fn App() -> impl IntoView {
    let (value, set_value) = create_signal(String::new());

    let update_value = move |ev: Event| {
        let v = event_target_value(&ev);
        set_value.set(v);
    };

    let message = use_message();

    let submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let v = value.get_untracked();
            if v.is_empty() {
                return;
            }
            let args = serde_wasm_bindgen::to_value(&SetArgs {
                key: "test",
                val: &v,
                info: ConnInfo {
                    name: "test".into(),
                    addr: Standalone("127.0.0.1".into(), 6379, 0),
                    read_only: false,
                    username: None,
                    password: None,
                    separator: None,
                    ssl: None,
                },
            }).unwrap();

            match invoke("set", args).await {
                Ok(_) => { message.create("success".into(), MessageVariant::Success, MessageOptions::default()) }
                Err(err) => {
                    let e = from_value::<Error>(err).unwrap();
                    message.create(format!("{}", e), MessageVariant::Error, MessageOptions::default())
                }
            }
        })
    };

    view! {
        <Layout has_sider=true>
            <LayoutSider style="background-color: #0078ff99; padding: 20px;">
                "Sider"
            </LayoutSider>
            <Layout>
                <LayoutHeader style="background-color: #0078ffaa; padding: 20px;">
                    "Header"
                </LayoutHeader>
                <Layout style="background-color: #0078ff88; padding: 20px;">
                        <form class="row" on:submit=submit>
                        <input
                            id="greet-input"
                            placeholder="Enter a name..."
                            on:input=update_value
                        />
                        <button type="submit">"Greet"</button>
                    </form>
                </Layout>
            </Layout>
        </Layout>
    }
}
