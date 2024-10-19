use leptos::*;
use wasm_bindgen::prelude::*;
use thaw::{Layout, LayoutHeader, LayoutSider};

#[wasm_bindgen]
extern "C" {
    // invoke without arguments
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[component]
pub fn App() -> impl IntoView {
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
                    "Content"
                </Layout>
            </Layout>
        </Layout>
    }
}
