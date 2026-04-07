// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn InstallSection() -> impl IntoView {
    view! {
        <section id="install" class="py-24 bg-surface-raised">
            <div class="mx-auto max-w-3xl px-4">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "Get started in " <span class="text-honey">"30 seconds"</span>
                    </h2>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-12 space-y-6">
                    <crate::components::scroll_reveal::ScrollReveal>
                        <div class="rounded-xl border border-honey/20 bg-surface p-6">
                            <h3 class="text-sm font-semibold text-comb mb-3">"1. Install"</h3>
                            <div class="space-y-2">
                                <div class="relative group rounded-lg bg-pollen p-4 font-mono text-sm text-bark">
                                    <span class="text-comb">"$ "</span>"cargo install commitbee"
                                    <button class="copy-btn absolute top-2 right-2 opacity-0 group-hover:opacity-100 focus:opacity-100 transition-opacity text-xs text-comb hover:text-honey" data-code="cargo install commitbee">"Copy"</button>
                                </div>
                                <div class="text-center text-xs text-comb">"or"</div>
                                <div class="relative group rounded-lg bg-pollen p-4 font-mono text-sm text-bark">
                                    <span class="text-comb">"$ "</span>"brew install sephyi/tap/commitbee"
                                    <button class="copy-btn absolute top-2 right-2 opacity-0 group-hover:opacity-100 focus:opacity-100 transition-opacity text-xs text-comb hover:text-honey" data-code="brew install sephyi/tap/commitbee">"Copy"</button>
                                </div>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>

                    <crate::components::scroll_reveal::ScrollReveal>
                        <div class="rounded-xl border border-honey/20 bg-surface p-6">
                            <h3 class="text-sm font-semibold text-comb mb-3">"2. Pull a model"</h3>
                            <div class="relative group rounded-lg bg-pollen p-4 font-mono text-sm text-bark">
                                <span class="text-comb">"$ "</span>"ollama pull qwen3.5:4b"
                                <button class="copy-btn absolute top-2 right-2 opacity-0 group-hover:opacity-100 focus:opacity-100 transition-opacity text-xs text-comb hover:text-honey" data-code="ollama pull qwen3.5:4b">"Copy"</button>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>

                    <crate::components::scroll_reveal::ScrollReveal>
                        <div class="rounded-xl border border-honey/20 bg-surface p-6">
                            <h3 class="text-sm font-semibold text-comb mb-3">"3. Commit"</h3>
                            <div class="relative group rounded-lg bg-pollen p-4 font-mono text-sm text-bark">
                                <div><span class="text-comb">"$ "</span>"git add src/feature.rs"</div>
                                <div><span class="text-comb">"$ "</span>"commitbee"</div>
                                <button class="copy-btn absolute top-2 right-2 opacity-0 group-hover:opacity-100 focus:opacity-100 transition-opacity text-xs text-comb hover:text-honey" data-code="git add src/feature.rs && commitbee">"Copy"</button>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>

                    <crate::components::scroll_reveal::ScrollReveal>
                        <p class="text-center text-comb">
                            "That's it. Works with zero configuration if Ollama is running."
                        </p>
                    </crate::components::scroll_reveal::ScrollReveal>
                </div>
            </div>
        </section>
    }
}
