// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn DocsTransitionSection() -> impl IntoView {
    view! {
        <section class="py-24 bg-surface/55 backdrop-blur-md">
            <div class="mx-auto max-w-5xl px-4">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "Dive deeper"
                    </h2>
                    <p class="mt-4 text-center text-comb max-w-xl mx-auto">
                        "Explore the full documentation to master CommitBee."
                    </p>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-12 grid grid-cols-1 gap-4 sm:grid-cols-2">
                    <DocLink slug="getting-started" title="Getting Started" description="Install and generate your first commit message"/>
                    <DocLink slug="configuration" title="Configuration" description="5-level config system, TOML files, environment variables"/>
                    <DocLink slug="llm-providers" title="LLM Providers" description="Ollama, OpenAI, and Anthropic setup"/>
                    <DocLink slug="architecture" title="Architecture" description="Deep dive into the pipeline internals"/>
                </div>
            </div>
        </section>
    }
}

#[component]
fn DocLink(slug: &'static str, title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <crate::components::scroll_reveal::ScrollReveal>
            <a
                href=format!("/docs/{slug}")
                class="block rounded-xl border border-honey/20 bg-surface-raised p-6 hover:border-honey/40 hover:shadow-lg hover:shadow-honey/5 transition-all group"
            >
                <h3 class="text-lg font-semibold text-bark group-hover:text-honey transition-colors">{title}</h3>
                <p class="mt-1 text-sm text-comb">{description}</p>
                <span class="mt-3 inline-block text-sm text-honey">"Read more →"</span>
            </a>
        </crate::components::scroll_reveal::ScrollReveal>
    }
}
