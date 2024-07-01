use leptos::*;
use wasm_bindgen::prelude::*;

use crate::components::app_button::{AddAppButton, AddScriptButton, AppButton, ScriptButton};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="flex flex-col bg-gray-800 text-white py-2 px-5 m-0 text-center h-screen w-screen">
            <div class="divider divider-start">Apps</div>
            <div class="grid grid-cols-4 gap-1.5">
                <AppButton name="League of Legends".to_string() />
                <AppButton name="League of Legends".to_string() />
                <AppButton name="League of Legends".to_string() />
                <AppButton name="League of Legends".to_string() />
                <AppButton name="League of Legends".to_string() />
                <AppButton name="Horizon Zero Dawn".to_string() />
                <AddAppButton />
            </div>
            <div class="divider divider-start">Scripts</div>
            <div class="grid grid-cols-1 gap-1.5">
                <ScriptButton name="League of Legends".to_string() />
                <ScriptButton name="League of Legends".to_string() />
                <AddScriptButton />
            </div>
        </main>
    }
}
