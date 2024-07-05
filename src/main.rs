mod app;
mod components;
mod module;

use crate::app::App;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
