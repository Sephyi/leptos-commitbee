// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::{footer::Footer, nav::Nav};
use crate::pages::{docs::DocsPage, landing::Landing, not_found::NotFound};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Nav/>
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=path!("/") view=Landing/>
                <Route path=path!("/docs/:slug") view=DocsPage/>
            </Routes>
            <Footer/>
        </Router>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href="/images/favicon.svg" type="image/svg+xml"/>
                <link rel="alternate icon" href="/images/favicon.ico"/>
                <link rel="apple-touch-icon" href="/images/apple-touch-icon.png"/>
                <link rel="preload" href="/fonts/Inter-Variable.woff2" as_="font" type_="font/woff2" crossorigin="anonymous"/>
                // Inline theme script: prevents FOUC
                <script>{r#"
                    (function(){
                        var t = localStorage.getItem('theme');
                        if (t === 'dark' || (!t && matchMedia('(prefers-color-scheme:dark)').matches)) {
                            document.documentElement.classList.add('dark');
                        }
                    })();
                "#}</script>
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body class="bg-surface text-bark antialiased">
                <App/>
            </body>
        </html>
    }
}
