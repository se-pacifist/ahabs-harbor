use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "primitives"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;

  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
  async fn emit(event: &str, payload: JsValue) -> JsValue;
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
  async fn listen(event: &str, callback: JsValue) -> JsValue;
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
  async fn once(event: &str, callback: JsValue) -> JsValue;
}

#[component]
pub fn Titlebar() -> impl IntoView {
  view! {
    <div class="w-full titlebar" data-tauri-drag-region />
  }
}

#[component]
pub fn Main(children: ChildrenFn) -> impl IntoView {
  view! {
    <main class="flex flex-col">
      { children() }
    </main>
  }
}
#[component]
pub fn Page(children: ChildrenFn) -> impl IntoView {
  view! {
    <div class="flex flex-col items-center justify-center h-screen">
      { children() }
    </div>
  }
}

#[component]
pub fn App() -> impl IntoView {
  view! {
    <Main>
      <Titlebar />
      <Page>
        <img src="/public/logo.png" style="width:300px" />
        <h2 class="p-6 text-4xl">"Ahabs Harbor"</h2>
        <p class="px-10 pb-10 text-center">"We'll catch that whale yet my boys!"</p>
        <ul>
          <li>"Containerd"</li>
          <li>"k3s"</li>
          <li>"Kubectl"</li>
        </ul>
      </Page>
    </Main>
  }
}
