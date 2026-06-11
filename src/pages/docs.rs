// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

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

                let page_url = format!("{}/docs/{}", crate::components::seo::BASE_URL, current_slug);
                let json_ld = serde_json::json!({
                    "@context": "https://schema.org",
                    "@graph": [
                        {
                            "@type": "TechArticle",
                            "headline": page.title,
                            "description": page.description,
                            "url": page_url,
                            "author": {"@type": "Person", "name": "Sephyi", "url": "https://sephy.io"}
                        },
                        {
                            "@type": "BreadcrumbList",
                            "itemListElement": [
                                {"@type": "ListItem", "position": 1, "name": "Docs",
                                 "item": format!("{}/docs/getting-started", crate::components::seo::BASE_URL)},
                                {"@type": "ListItem", "position": 2, "name": page.section},
                                {"@type": "ListItem", "position": 3, "name": page.title, "item": page_url}
                            ]
                        }
                    ]
                })
                .to_string();

                view! {
                    <crate::components::seo::SeoMeta
                        title=format!("{} - CommitBee Docs", page.title)
                        description=page.description
                        path=format!("/docs/{}", current_slug)
                        article=true
                    />
                    <script type="application/ld+json" inner_html=json_ld></script>

                    <div id="main-content" class="flex min-h-screen">
                        // Left sidebar (sticky, full-height)
                        <div class="hidden lg:block">
                            <DocSidebar current_slug=current_slug.clone()/>
                        </div>

                        // Main content — glass card so the shader background
                        // shows through faintly while the article stays
                        // readable.
                        <main class="flex-1 min-w-0 px-6 py-8 lg:px-10 bg-surface/65 backdrop-blur-md">
                            // Breadcrumbs
                            <nav class="mb-6 text-sm text-comb" aria-label="Breadcrumb">
                                <a href="/docs/getting-started" class="transition-colors hover:text-honey">"Docs"</a>
                                " / "
                                <span class="text-comb">{page.section}</span>
                                " / "
                                <span class="font-medium text-bark">{page.title}</span>
                            </nav>

                            // Rendered markdown content
                            <article class="prose max-w-none">
                                <div inner_html=page.html_content/>
                            </article>

                            // Activate copy buttons on code blocks
                            <crate::components::code_block::CodeBlockActivator/>

                            // Prev/next navigation
                            <nav class="flex justify-between pt-6 mt-12 border-t border-honey/10">
                                {prev.map(|p| view! {
                                    <a href=format!("/docs/{}", p.slug) class="group">
                                        <span class="text-xs text-comb">"Previous"</span>
                                        <div class="text-sm font-medium transition-colors text-bark group-hover:text-honey">
                                            "← " {p.title}
                                        </div>
                                    </a>
                                })}
                                <div/>
                                {next.map(|p| view! {
                                    <a href=format!("/docs/{}", p.slug) class="text-right group">
                                        <span class="text-xs text-comb">"Next"</span>
                                        <div class="text-sm font-medium transition-colors text-bark group-hover:text-honey">
                                            {p.title} " →"
                                        </div>
                                    </a>
                                })}
                            </nav>
                        </main>

                        // Right TOC + scroll-spy highlighter
                        <DocToc headings=page.headings/>
                        <crate::components::toc_highlighter::TocHighlighter/>
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
                            <a href="/docs/getting-started" class="inline-block mt-4 text-honey hover:underline">
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
