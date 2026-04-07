// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn PipelineSection() -> impl IntoView {
    view! {
        <section id="pipeline" class="py-24 bg-surface/55 backdrop-blur-md">
            <div class="max-w-5xl px-4 mx-auto">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "A " <span class="text-honey">"7-stage pipeline"</span> " from diff to commit"
                    </h2>
                    <p class="max-w-2xl mx-auto mt-4 text-center text-comb">
                        "Watch how CommitBee processes your code, step by step."
                    </p>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-16">
                    <crate::components::pipeline_demo::PipelineDemo/>
                </div>
            </div>
        </section>
    }
}
