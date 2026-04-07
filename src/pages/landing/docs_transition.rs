// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn DocsTransitionSection() -> impl IntoView {
    view! {
        <section class="py-24 bg-surface/55 backdrop-blur-md">
            <div class="max-w-5xl px-4 mx-auto">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "Dive deeper"
                    </h2>
                    <p class="max-w-xl mx-auto mt-4 text-center text-comb">
                        "Explore the full documentation to master CommitBee."
                    </p>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="grid grid-cols-1 gap-4 mt-12 sm:grid-cols-2">
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
                class="block p-6 transition-all border rounded-xl border-honey/20 bg-surface-raised hover:border-honey/40 hover:shadow-lg hover:shadow-honey/5 group"
            >
                <h3 class="text-lg font-semibold transition-colors text-bark group-hover:text-honey">{title}</h3>
                <p class="mt-1 text-sm text-comb">{description}</p>
                <span class="inline-block mt-3 text-sm text-honey">"Read more →"</span>
            </a>
        </crate::components::scroll_reveal::ScrollReveal>
    }
}
