mod app;
mod components;

use crate::app::App;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
