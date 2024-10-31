use api::error::Error;
use api::ConnAddr::Standalone;
use api::ConnInfo;
use leptos::ev::{Event, SubmitEvent};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Serialize;
use serde_wasm_bindgen::from_value;
use thaw::{Toast, ToastIntent, ToastOptions, ToastPosition, ToastTitle, ToasterInjection};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // invoke without arguments
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize)]
struct SetArgs<'a> {
    key: &'a str,
    val: &'a str,
    info: ConnInfo,
}

#[derive(Serialize)]
struct GetArgs<'a> {
    key: &'a str,
    info: ConnInfo,
}

#[component]
pub fn App() -> impl IntoView {
    let (value, set_value) = signal(String::new());

    let update_value = move |ev: Event| {
        let v = event_target_value(&ev);
        set_value.set(v);
    };

    let toaster = ToasterInjection::expect_context();

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
                Ok(_) => {
                    toaster.dispatch_toast(move || view! {
                    <Toast>
                        <ToastTitle>success!</ToastTitle>
                    </Toast>
                }, ToastOptions::default().with_intent(ToastIntent::Success).with_position(ToastPosition::Top))
                }
                Err(err) => {
                    let e = from_value::<Error>(err).unwrap();
                    toaster.dispatch_toast(move || view! {
                        <Toast>
                            <ToastTitle>{ e.msg }</ToastTitle>
                        </Toast>
                    }, ToastOptions::default().with_intent(ToastIntent::Error).with_position(ToastPosition::Top))
                }
            }
        })
    };

    view! {
        <div class="layout">
            <div class="sider">
                <div class="resize-bar"></div>
                <div class="resize-line"></div>
                <div class="resize-content">
                    "test"
                </div>                                            
            </div>
            <div class="content">
                <form class="row" on:submit=submit>
                    <input
                        id="greet-input"
                        placeholder="Enter a name..."
                        on:input=update_value
                    />
                    <button type="submit">"Greet"</button>
                </form>
            </div>
        </div>
    }
}
