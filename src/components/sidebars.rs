//! Sidebar components: Nav (left) and Aside (right).

use yew::prelude::*;

use crate::lorem;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="site-nav">
            <h2>{ "Navigation" }</h2>
            <ul>
                <li><a href="#home">{ "Home" }</a></li>
                <li><a href="#about">{ "About" }</a></li>
                <li><a href="#services">{ "Services" }</a></li>
                <li><a href="#portfolio">{ "Portfolio" }</a></li>
                <li><a href="#blog">{ "Blog" }</a></li>
                <li><a href="#contact">{ "Contact" }</a></li>
            </ul>
        </nav>
    }
}

fn quick_links() -> Html {
    html! {
        <section class="aside-section">
            <h3>{ "Quick Links" }</h3>
            <ul>
                <li><a href="#home">{ "Back to Top" }</a></li>
                <li><a href="https://yew.rs" target="_blank" rel="noopener">{ "Yew Docs" }</a></li>
                <li><a href="https://www.rust-lang.org" target="_blank" rel="noopener">{ "Rust" }</a></li>
                <li><a href="https://developer.mozilla.org" target="_blank" rel="noopener">{ "MDN" }</a></li>
            </ul>
        </section>
    }
}

fn recent_posts() -> Html {
    html! {
        <section class="aside-section">
            <h3>{ "Recent Posts" }</h3>
            <ul class="recent-posts">
                <li>
                    <a href="#blog">{ "Getting Started with Yew" }</a>
                    <span class="date">{ "Jan 15" }</span>
                </li>
                <li>
                    <a href="#blog">{ "CSS Theming Strategies" }</a>
                    <span class="date">{ "Jan 10" }</span>
                </li>
                <li>
                    <a href="#blog">{ "Rust for Web Dev" }</a>
                    <span class="date">{ "Jan 5" }</span>
                </li>
            </ul>
        </section>
    }
}

#[function_component(Aside)]
pub fn aside() -> Html {
    html! {
        <aside class="site-aside">
            { quick_links() }
            <section class="aside-section">
                <h3>{ "Info Box" }</h3>
                <p>{ lorem::SHORT }</p>
            </section>
            { recent_posts() }
            <section class="aside-section">
                <h3>{ "Tags" }</h3>
                <div class="tag-cloud">
                    <span class="tag">{ "rust" }</span>
                    <span class="tag">{ "yew" }</span>
                    <span class="tag">{ "wasm" }</span>
                    <span class="tag">{ "css" }</span>
                    <span class="tag">{ "html5" }</span>
                    <span class="tag">{ "web" }</span>
                </div>
            </section>
        </aside>
    }
}
