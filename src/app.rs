use crate::icon::Icon;
use icondata_fa::{FaArrowsRotateSolid, FaPowerOffSolid};
use leptos::{prelude::*, task::spawn_local};
use leptos_meta::MetaTags;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Redirect, Route, Router},
    path,
    static_routes::StaticRoute,
    SsrMode,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let fallback = || view! { "Page not found." }.into_view();

    view! {
        <Stylesheet id="leptos" href="/pkg/ShutRemote.css"/>
        <Title text="Welcome to Leptos"/>
        <Meta name="color-scheme" content="dark light"/>
        <Router>
            <main>
                <FlatRoutes fallback>
                    <Route
                        path=path!("/")
                        view=HomePage
                        ssr=SsrMode::Static(
                            StaticRoute::new(),
                        )
                    />

                    <Route
                        path=path!("/about")
                        view=move || view! { <Redirect path="/"/> }
                        ssr=SsrMode::Static(StaticRoute::new())
                    />
                </FlatRoutes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="main-container">
            <div class="card">
                <h1 class="rubik-h1 text-center">ShutRemote</h1>
                <div class="button-container">
                    <button class="action-button shutdown-button" on:click=move |_| {
                        spawn_local(async {shutdown().await;});
                    }>
                        <Icon icon=icondata_core::Icon::from(FaPowerOffSolid) attr:class="button-icon"/>
                        <span class="button-text">Shutdown</span>
                    </button>
                    <button class="action-button reboot-button" on:click=move |_| {
                        spawn_local(async {reboot().await;});
                    }>
                        <Icon icon=icondata_core::Icon::from(FaArrowsRotateSolid) attr:class="button-icon"/>
                        <span class="button-text">Reboot</span>
                    </button>
                </div>
            </div>
        </div>
    }
}

#[server]
pub async fn shutdown() -> Result<(), ServerFnError> {
    println!("Shutdown computer");
    system_shutdown::shutdown().unwrap();
    Ok(())
}

#[server]
pub async fn reboot() -> Result<(), ServerFnError> {
    println!("Reboot computer");
    system_shutdown::force_reboot().unwrap();
    Ok(())
}
