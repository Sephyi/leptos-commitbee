use leptos::prelude::*;
use leptos_meta::*;

use crate::components::{doc_sidebar::DocSidebar, doc_toc::DocToc};
use crate::content::loader;

#[component]
pub fn DocsPage() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default().to_string());

    let page_view = move || {
        let current_slug = slug();
        match loader::get_page(&current_slug) {
            Some(page) => {
                let (prev, next) = loader::get_adjacent(&current_slug);

                view! {
                    <Title text=format!("{} - CommitBee Docs", page.title)/>
                    <Meta name="description" content=page.description/>
                    <Meta property="og:title" content=format!("{} - CommitBee Docs", page.title)/>
                    <Meta property="og:description" content=page.description/>

                    <div class="flex min-h-screen">
                        // Left sidebar
                        <div class="hidden lg:block">
                            <DocSidebar current_slug=current_slug.clone()/>
                        </div>

                        // Main content
                        <main class="flex-1 min-w-0 px-6 py-8 lg:px-12">
                            // Breadcrumbs
                            <nav class="mb-6 text-sm text-comb" aria-label="Breadcrumb">
                                <a href="/docs/getting-started" class="hover:text-honey transition-colors">"Docs"</a>
                                " / "
                                <span class="text-comb">{page.section}</span>
                                " / "
                                <span class="text-bark font-medium">{page.title}</span>
                            </nav>

                            // Rendered markdown content
                            <article class="prose max-w-prose">
                                <div inner_html=page.html_content/>
                            </article>

                            // Activate copy buttons on code blocks
                            <crate::components::code_block::CodeBlockActivator/>

                            // Prev/next navigation
                            <nav class="mt-12 flex justify-between border-t border-honey/10 pt-6">
                                {prev.map(|p| view! {
                                    <a href=format!("/docs/{}", p.slug) class="group">
                                        <span class="text-xs text-comb">"Previous"</span>
                                        <div class="text-sm font-medium text-bark group-hover:text-honey transition-colors">
                                            "← " {p.title}
                                        </div>
                                    </a>
                                })}
                                <div/>
                                {next.map(|p| view! {
                                    <a href=format!("/docs/{}", p.slug) class="group text-right">
                                        <span class="text-xs text-comb">"Next"</span>
                                        <div class="text-sm font-medium text-bark group-hover:text-honey transition-colors">
                                            {p.title} " →"
                                        </div>
                                    </a>
                                })}
                            </nav>
                        </main>

                        // Right TOC
                        <DocToc headings=page.headings/>
                    </div>
                }.into_any()
            }
            None => {
                view! {
                    <Title text="Page Not Found - CommitBee Docs"/>
                    <div class="flex items-center justify-center min-h-[50vh]">
                        <div class="text-center">
                            <h1 class="text-4xl font-bold text-bark">"404"</h1>
                            <p class="mt-2 text-comb">"Documentation page not found."</p>
                            <a href="/docs/getting-started" class="mt-4 inline-block text-honey hover:underline">
                                "Go to Getting Started"
                            </a>
                        </div>
                    </div>
                }.into_any()
            }
        }
    };

    view! {
        {page_view}
    }
}
