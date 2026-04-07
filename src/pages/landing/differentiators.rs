// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn DifferentiatorsSection() -> impl IntoView {
    view! {
        <section class="py-24 bg-surface/55 backdrop-blur-md">
            <div class="max-w-6xl px-4 mx-auto">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "What sets CommitBee " <span class="text-honey">"apart"</span>
                    </h2>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="grid grid-cols-1 gap-8 mt-16 md:grid-cols-3 reveal-stagger">
                    <FeatureCard
                        title="Tree-sitter Semantic Analysis"
                        description="It reads your code, not just your diffs. Extracts 10 symbol types across 10 languages and maps diff hunks to their spans."
                        icon="🌳"
                    />
                    <FeatureCard
                        title="Commit Splitting"
                        description="It detects mixed concerns and splits them into separate, well-typed commits using diff-shape fingerprinting."
                        icon="🔀"
                    />
                    <FeatureCard
                        title="25-Pattern Secret Scanning"
                        description="It catches leaked credentials before they reach any LLM. 25 patterns across 13 categories, fully customizable."
                        icon="🔒"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(
    title: &'static str,
    description: &'static str,
    icon: &'static str,
) -> impl IntoView {
    view! {
        <crate::components::scroll_reveal::ScrollReveal>
            <div class="p-6 transition-all border rounded-xl border-honey/20 bg-surface hover:border-honey/40 hover:shadow-lg hover:shadow-honey/5">
                <div class="mb-4 text-3xl">{icon}</div>
                <h3 class="text-lg font-semibold text-bark">{title}</h3>
                <p class="mt-2 text-sm text-comb">{description}</p>
            </div>
        </crate::components::scroll_reveal::ScrollReveal>
    }
}
