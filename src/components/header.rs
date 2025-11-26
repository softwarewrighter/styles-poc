//! Header component with branding and theme selector.

use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub theme: String,
    pub on_theme_change: Callback<String>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let on_change = props.on_theme_change.clone();

    let onchange = Callback::from(move |e: Event| {
        let target = e.target();
        let select = target
            .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
            .expect("event target should be a select element");
        on_change.emit(select.value());
    });

    html! {
        <header class="site-header">
            <div class="header-brand">
                <h1>{ "Styles POC" }</h1>
                <span class="header-tagline">{ "HTML5 Layout Demo" }</span>
            </div>
            <div class="theme-selector">
                <label for="theme-select">{ "Theme: " }</label>
                <select id="theme-select" value={props.theme.clone()} {onchange}>
                    <option value="simple-light" selected={props.theme == "simple-light"}>
                        { "Simple Light" }
                    </option>
                    <option value="simple-dark" selected={props.theme == "simple-dark"}>
                        { "Simple Dark" }
                    </option>
                    <option value="improved-light" selected={props.theme == "improved-light"}>
                        { "Improved Light" }
                    </option>
                    <option value="improved-dark" selected={props.theme == "improved-dark"}>
                        { "Improved Dark" }
                    </option>
                    <option value="wild-light" selected={props.theme == "wild-light"}>
                        { "Wild Light" }
                    </option>
                    <option value="wild-dark" selected={props.theme == "wild-dark"}>
                        { "Wild Dark" }
                    </option>
                    <option value="terminal-dark" selected={props.theme == "terminal-dark"}>
                        { "Terminal Dark" }
                    </option>
                </select>
            </div>
        </header>
    }
}
