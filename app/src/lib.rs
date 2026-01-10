mod api;
mod routes;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
use routes::*;
use singlestage::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // This tells the client where to find the server in csr builds
    #[cfg(feature = "csr")]
    leptos::server_fn::client::set_server_url("http://localhost:3000");

    view! {
        {
            #[cfg(not(feature = "csr"))]
            // In csr mode the css file is sourced in index.html
            view! { <leptos_meta::Stylesheet id="leptos" href="/pkg/{{ project-name }}.css" /> }
        }

        <Title text="{{ project-name }}" />

        <ThemeProvider>
            <header>
                <a href="/">
                    <h1 class="text-2xl font-semibold m-4">
                        "{{ project-name }}"
                        {if cfg!(feature = "csr") {
                            " csr"
                        } else if cfg!(feature = "ssr") {
                            " ssr"
                        } else {
                            ""
                        }}
                    </h1>
                </a>
            </header>

            <Separator />

            <Router>
                <Routes fallback=NotFound>
                    <Route path=StaticSegment("") view=Home />
                </Routes>
            </Router>
        </ThemeProvider>
    }
}
