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

/// Redirects /docs to /docs/getting-started via meta refresh (works without JS).
#[component]
fn DocsRedirect() -> impl IntoView {
    view! {
        <meta http-equiv="refresh" content="0;url=/docs/getting-started"/>
        <p>"Redirecting to "<a href="/docs/getting-started">"Getting Started"</a>"..."</p>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Nav/>
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=path!("/") view=Landing/>
                <Route path=path!("/docs") view=DocsRedirect/>
                <Route path=path!("/docs/:slug") view=DocsPage/>
            </Routes>
            <Footer/>
        </Router>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    let pkg_path = &options.site_pkg_dir;
    let css_href = resolve_css_href(&options, pkg_path);

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href="/images/favicon.svg" type="image/svg+xml"/>
                <link rel="alternate icon" href="/images/favicon.svg"/>
                <link rel="preload" href="/fonts/Inter-Variable.woff2" r#as="font" r#type="font/woff2" crossorigin="anonymous"/>
                <link rel="stylesheet" href=css_href/>
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
                // Inline scroll-reveal observer: runs before WASM, no hydration dependency
                <script>{r#"
                    (function(){
                        var vh=window.innerHeight;
                        var o=new IntersectionObserver(function(e){
                            e.forEach(function(x){if(x.isIntersecting)x.target.classList.add('visible')});
                        },{threshold:0.1});
                        var s='.reveal:not(.visible),.reveal-left:not(.visible),.reveal-right:not(.visible),.reveal-scale:not(.visible)';
                        function scan(){
                            document.querySelectorAll(s).forEach(function(el){
                                var r=el.getBoundingClientRect();
                                if(r.top<vh&&r.bottom>0){
                                    el.style.transition='none';
                                    el.classList.add('visible');
                                    void el.offsetHeight;
                                    el.style.removeProperty('transition');
                                } else o.observe(el);
                            });
                        }
                        scan();
                        new MutationObserver(scan).observe(document.body,{childList:true,subtree:true});
                    })();
                "#}</script>
                // Link prefetching: fetch same-origin pages on hover for instant navigation
                <script>{r#"
                    (function(){
                        var c={};
                        document.addEventListener('pointerenter',function(e){
                            var a=e.target.closest('a');
                            if(!a||!a.href||a.target||a.origin!==location.origin||c[a.pathname]) return;
                            c[a.pathname]=1;
                            var l=document.createElement('link');
                            l.rel='prefetch';l.href=a.href;
                            document.head.appendChild(l);
                        },true);
                    })();
                "#}</script>
                <noscript>
                    <style>".reveal,.reveal-left,.reveal-right,.reveal-scale{opacity:1!important;transform:none!important}"</style>
                </noscript>
            </body>
        </html>
    }
}

fn resolve_css_href(options: &LeptosOptions, pkg_path: &str) -> String {
    let mut css_file = options.output_name.to_string();
    if options.hash_files {
        let hash_path = std::env::current_exe()
            .map(|path| path.parent().map(|p| p.to_path_buf()).unwrap_or_default())
            .unwrap_or_default()
            .join(options.hash_file.as_ref());
        if let Ok(hashes) = std::fs::read_to_string(&hash_path) {
            for line in hashes.lines() {
                let line = line.trim();
                if let Some((file, hash)) = line.split_once(':') {
                    if file == "css" {
                        css_file.push_str(&format!(".{}", hash.trim()));
                    }
                }
            }
        }
    }
    format!("/{pkg_path}/{css_file}.css")
}
