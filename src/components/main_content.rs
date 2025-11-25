//! Main content area with multiple sections demonstrating HTML5 elements.

use yew::prelude::*;

use crate::lorem;

fn home_section() -> Html {
    html! {
        <section class="content-section" id="home">
            <h2>{ "Welcome" }</h2>
            <article class="content-article">
                <h3>{ "About This Demo" }</h3>
                <p>{ lorem::MEDIUM }</p>
                <p>{ lorem::PARAGRAPH_1 }</p>
            </article>
            <article class="content-article">
                <h3>{ "HTML5 Semantic Elements" }</h3>
                <p>{ "This page demonstrates the proper use of HTML5 semantic layout elements:" }</p>
                <ul>
                    <li><code>{ "<header>" }</code>{ " - The page header with branding and navigation" }</li>
                    <li><code>{ "<nav>" }</code>{ " - Navigation links in the sidebar" }</li>
                    <li><code>{ "<main>" }</code>{ " - The primary content area" }</li>
                    <li><code>{ "<section>" }</code>{ " - Thematic groupings of content" }</li>
                    <li><code>{ "<article>" }</code>{ " - Self-contained content pieces" }</li>
                    <li><code>{ "<aside>" }</code>{ " - Supplementary content in the sidebar" }</li>
                    <li><code>{ "<footer>" }</code>{ " - The page footer" }</li>
                </ul>
            </article>
        </section>
    }
}

fn about_section() -> Html {
    html! {
        <section class="content-section" id="about">
            <h2>{ "About" }</h2>
            <article class="content-article">
                <h3>{ "Our Mission" }</h3>
                <p>{ lorem::LONG }</p>
            </article>
            <article class="content-article">
                <h3>{ "Our History" }</h3>
                <p>{ lorem::PARAGRAPH_2 }</p>
                <p>{ lorem::PARAGRAPH_3 }</p>
            </article>
        </section>
    }
}

fn services_section() -> Html {
    html! {
        <section class="content-section" id="services">
            <h2>{ "Services" }</h2>
            <article class="content-article">
                <h3>{ "Web Development" }</h3>
                <p>{ lorem::MEDIUM }</p>
                <h4>{ "Technologies We Use" }</h4>
                <ul>
                    <li>{ "Rust & WebAssembly" }</li>
                    <li>{ "Yew Framework" }</li>
                    <li>{ "Modern CSS" }</li>
                    <li>{ "Responsive Design" }</li>
                </ul>
            </article>
            <article class="content-article">
                <h3>{ "Consulting" }</h3>
                <p>{ lorem::SHORT }</p>
                <blockquote>
                    { "\"The best way to predict the future is to create it.\" - Peter Drucker" }
                </blockquote>
            </article>
        </section>
    }
}

fn portfolio_section() -> Html {
    html! {
        <section class="content-section" id="portfolio">
            <h2>{ "Portfolio" }</h2>
            <article class="content-article">
                <h3>{ "Project Alpha" }</h3>
                <p>{ lorem::PARAGRAPH_1 }</p>
                <figure>
                    <div class="placeholder-image">{ "[Image Placeholder]" }</div>
                    <figcaption>{ "A screenshot of Project Alpha" }</figcaption>
                </figure>
            </article>
            <article class="content-article">
                <h3>{ "Project Beta" }</h3>
                <p>{ lorem::PARAGRAPH_2 }</p>
            </article>
        </section>
    }
}

fn blog_section() -> Html {
    html! {
        <section class="content-section" id="blog">
            <h2>{ "Blog" }</h2>
            <article class="content-article blog-post">
                <header>
                    <h3>{ "Getting Started with Yew" }</h3>
                    <p class="post-meta">
                        <time datetime="2024-01-15">{ "January 15, 2024" }</time>
                        { " by " }<span class="author">{ "Demo Author" }</span>
                    </p>
                </header>
                <p>{ lorem::MEDIUM }</p>
                <footer>
                    <p class="post-tags">
                        { "Tags: " }
                        <span class="tag">{ "rust" }</span>
                        <span class="tag">{ "yew" }</span>
                        <span class="tag">{ "wasm" }</span>
                    </p>
                </footer>
            </article>
            <article class="content-article blog-post">
                <header>
                    <h3>{ "CSS Theming Strategies" }</h3>
                    <p class="post-meta">
                        <time datetime="2024-01-10">{ "January 10, 2024" }</time>
                        { " by " }<span class="author">{ "Demo Author" }</span>
                    </p>
                </header>
                <p>{ lorem::PARAGRAPH_3 }</p>
            </article>
        </section>
    }
}

fn contact_section() -> Html {
    html! {
        <section class="content-section" id="contact">
            <h2>{ "Contact" }</h2>
            <article class="content-article">
                <h3>{ "Get In Touch" }</h3>
                <p>{ lorem::SHORT }</p>
                <address>
                    <p>{ "Email: demo@example.com" }</p>
                    <p>{ "Phone: (555) 123-4567" }</p>
                    <p>{ "Address: 123 Demo Street, Example City, EX 12345" }</p>
                </address>
            </article>
        </section>
    }
}

#[function_component(MainContent)]
pub fn main_content() -> Html {
    html! {
        <main class="site-main">
            { home_section() }
            { about_section() }
            { services_section() }
            { portfolio_section() }
            { blog_section() }
            { contact_section() }
        </main>
    }
}
