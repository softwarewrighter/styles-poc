//! Styles POC - A Yew/WASM demo of HTML5 semantic layouts with theme switching.

mod components;
mod lorem;

use wasm_bindgen::JsCast;
use yew::prelude::*;

use components::{
    footer::Footer,
    header::Header,
    main_content::MainContent,
    sidebars::{Aside, Nav},
};

#[function_component(App)]
fn app() -> Html {
    let theme = use_state(|| "simple-light".to_string());

    let on_theme_change = {
        let theme = theme.clone();
        Callback::from(move |new_theme: String| {
            set_theme(&new_theme);
            theme.set(new_theme);
        })
    };

    html! {
        <div class="page-layout">
            <Header theme={(*theme).clone()} on_theme_change={on_theme_change} />
            <Nav />
            <MainContent />
            <Aside />
            <Footer />
        </div>
    }
}

fn set_theme(theme: &str) {
    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");

    if let Some(link) = document.get_element_by_id("theme-stylesheet") {
        if let Ok(link) = link.dyn_into::<web_sys::HtmlLinkElement>() {
            link.set_href(&format!("styles/{theme}.css"));
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
