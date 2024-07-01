use leptos::*;

#[component]
pub fn AppButton(name: String) -> impl IntoView {
    view! {
        <button class="btn h-20 w-20 bg-sky-700 text-sm rounded-lg font-semibold text-white text-wrap ">
            {name}
        </button>
    }
}

#[component]
pub fn AddAppButton() -> impl IntoView {
    view! {
        <button class="btn h-20 w-20 bg-gray-700 text-sm rounded-lg font-semibold text-white text-wrap ">
            "+"
        </button>
    }
}

#[component]
pub fn ScriptButton(name: String) -> impl IntoView {
    view! {
        <button class="btn h-10 w-full bg-amber-600 text-sm rounded-lg font-semibold text-white text-wrap ">
            {name}
        </button>
    }
}

#[component]
pub fn AddScriptButton() -> impl IntoView {
    view! {
        <button class="btn h-10 w-full bg-gray-700 text-sm rounded-lg font-semibold text-white text-wrap ">
            "+"
        </button>
    }
}
