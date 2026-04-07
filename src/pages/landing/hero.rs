// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="relative min-h-screen flex items-center justify-center overflow-hidden">
            <div class="absolute inset-0 hex-bg"/>

            <div class="relative mx-auto max-w-4xl px-4 text-center">
                <crate::components::scroll_reveal::ScrollReveal class="reveal">
                    <h1 class="text-4xl font-bold tracking-tight text-bark sm:text-6xl lg:text-7xl">
                        "The commit message generator that "
                        <span class="text-honey">"actually understands"</span>
                        " your code."
                    </h1>
                </crate::components::scroll_reveal::ScrollReveal>

                <crate::components::scroll_reveal::ScrollReveal class="reveal">
                    <p class="mt-6 text-lg text-comb sm:text-xl max-w-2xl mx-auto">
                        "CommitBee parses your code with tree-sitter, maps diff hunks to symbol spans, and gives the LLM structured semantic context."
                    </p>
                </crate::components::scroll_reveal::ScrollReveal>

                <crate::components::scroll_reveal::ScrollReveal class="reveal">
                    <div class="mt-10 flex items-center justify-center gap-4">
                        <a
                            href="#install"
                            class="rounded-lg bg-honey px-6 py-3 text-sm font-semibold text-white shadow-lg shadow-honey/25 hover:bg-honey-dark transition-colors"
                        >
                            "Get Started"
                        </a>
                        <a
                            href="#pipeline"
                            class="rounded-lg border border-honey/30 px-6 py-3 text-sm font-semibold text-bark hover:bg-honey/5 transition-colors"
                        >
                            "See How It Works"
                        </a>
                    </div>
                </crate::components::scroll_reveal::ScrollReveal>

                // Terminal mockup
                <crate::components::scroll_reveal::ScrollReveal class="reveal-scale">
                    <div class="mt-16 mx-auto max-w-2xl rounded-xl border border-honey/20 bg-surface-raised shadow-2xl shadow-honey/5 overflow-hidden">
                        <div class="flex items-center gap-2 px-4 py-3 bg-pollen/50 border-b border-honey/10">
                            <div class="w-3 h-3 rounded-full bg-red-400/60"/>
                            <div class="w-3 h-3 rounded-full bg-yellow-400/60"/>
                            <div class="w-3 h-3 rounded-full bg-green-400/60"/>
                            <span class="ml-2 text-xs text-comb">"~/project"</span>
                        </div>
                        <div class="p-4 font-mono text-sm text-bark text-left">
                            <div class="text-comb">"$ commitbee"</div>
                            <div class="mt-2 text-comb">"Analyzing 3 staged files..."</div>
                            <div class="text-comb">"Extracting symbols (tree-sitter)..."</div>
                            <div class="mt-2">"feat(auth): add JWT token refresh with configurable expiry"</div>
                            <div class="mt-1 text-comb">"Commit? (Y/n) " <span class="terminal-cursor">" "</span></div>
                        </div>
                    </div>
                </crate::components::scroll_reveal::ScrollReveal>
            </div>
        </section>
    }
}
