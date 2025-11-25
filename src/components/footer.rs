//! Footer component.
//!
//! Contains: Copyright, License Link, Repository Link, Build Host, Build Commit, Build Time

use yew::prelude::*;

mod build_info {
    pub const BUILD_HOST: &str = env!("BUILD_HOST");
    pub const BUILD_COMMIT: &str = env!("BUILD_COMMIT");
    pub const BUILD_TIME: &str = env!("BUILD_TIME");
}

// Footer text constants for sw-checklist detection
const COPYRIGHT_TEXT: &str = "Copyright 2024 Software Wrighter LLC";
const LICENSE_LINK_TEXT: &str = "License Link";
const REPOSITORY_LINK_TEXT: &str = "Repository Link";

#[function_component(Footer)]
pub fn footer() -> Html {
    let build_host = format!("Build Host: {}", build_info::BUILD_HOST);
    let build_commit = format!("Build Commit: {}", build_info::BUILD_COMMIT);
    let build_time = format!("Build Time: {}", build_info::BUILD_TIME);

    html! {
        <footer class="site-footer">
            <div class="footer-content">
                <div class="footer-left">
                    <p class="copyright">{ COPYRIGHT_TEXT }</p>
                    <p class="links">
                        <a href="LICENSE" target="_blank">{ LICENSE_LINK_TEXT }</a>
                        { " | " }
                        <a href="https://github.com/softwarewrighter/styles-poc"
                           target="_blank" rel="noopener">{ REPOSITORY_LINK_TEXT }</a>
                    </p>
                </div>
                <div class="footer-right">
                    <p class="build-info">{ build_host }{ " | " }{ build_commit }</p>
                    <p class="build-time">{ build_time }</p>
                </div>
            </div>
        </footer>
    }
}
