use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};
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

    view! {
        <Stylesheet id="leptos" href="/pkg/{{ project-name }}.css" />

        // sets the document title
        <Title text="{{ project-name }}" />

        // content for this welcome page
        <ThemeProvider>
            <Router>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </Router>
        </ThemeProvider>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <header>
            <h1 class="text-2xl font-semibold m-4">"{{ project-name }}"</h1>
        </header>

        <Separator />

        <main class="mx-4">
            <Item class="w-fit" variant="outline">
                <ItemContent>
                    <ItemTitle class="text-2xl font-semibold">
                        {if cfg!(feature = "csr") {
                            "csr"
                        } else if cfg!(feature = "hydrate") {
                            "hydrate"
                        } else if cfg!(feature = "ssr") {
                            "ssr"
                        } else {
                            "else"
                        }} " mode"
                    </ItemTitle>
                    <ItemActions>
                        <Button on:click=on_click>"Click Me: " {count}</Button>
                    </ItemActions>
                </ItemContent>
            </Item>
        </main>

        <Separator />
    }
}
