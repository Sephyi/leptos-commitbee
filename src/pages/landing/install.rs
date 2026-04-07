// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn InstallSection() -> impl IntoView {
    view! {
        <section id="install" class="py-24 bg-surface/55 backdrop-blur-md">
            <div class="max-w-3xl px-4 mx-auto">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "Get started in " <span class="text-honey">"30 seconds"</span>
                    </h2>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-12 space-y-6">
                    <crate::components::scroll_reveal::ScrollReveal>
                        <div class="p-6 border rounded-xl border-honey/20 bg-surface">
                            <h3 class="mb-3 text-sm font-semibold text-comb">"1. Install"</h3>
                            <div class="space-y-2">
                                <div class="relative p-4 font-mono text-sm rounded-lg group bg-pollen text-bark">
                                    <span class="text-comb">"$ "</span>"cargo install commitbee"
                                    <button class="absolute text-xs transition-opacity opacity-0 copy-btn top-2 right-2 group-hover:opacity-100 focus:opacity-100 text-comb hover:text-honey" data-code="cargo install commitbee">"Copy"</button>
                                </div>
                                <div class="text-xs text-center text-comb">"or"</div>
                                <div class="relative p-4 font-mono text-sm rounded-lg group bg-pollen text-bark">
                                    <span class="text-comb">"$ "</span>"cargo install --git https://github.com/Sephyi/commitbee --branch development commitbee"
                                    <button class="absolute text-xs transition-opacity opacity-0 copy-btn top-2 right-2 group-hover:opacity-100 focus:opacity-100 text-comb hover:text-honey" data-code="cargo install --git https://github.com/Sephyi/commitbee --branch development commitbee">"Copy"</button>
                                </div>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>

                    <crate::components::scroll_reveal::ScrollReveal>
                        <div class="p-6 border rounded-xl border-honey/20 bg-surface">
                            <h3 class="mb-3 text-sm font-semibold text-comb">"2. Pull a model"</h3>
                            <div class="relative p-4 font-mono text-sm rounded-lg group bg-pollen text-bark">
                                <span class="text-comb">"$ "</span>"ollama pull qwen3.5:4b"
                                <button class="absolute text-xs transition-opacity opacity-0 copy-btn top-2 right-2 group-hover:opacity-100 focus:opacity-100 text-comb hover:text-honey" data-code="ollama pull qwen3.5:4b">"Copy"</button>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>

                    <crate::components::scroll_reveal::ScrollReveal>
                        <div class="p-6 border rounded-xl border-honey/20 bg-surface">
                            <h3 class="mb-3 text-sm font-semibold text-comb">"3. Commit"</h3>
                            <div class="relative p-4 font-mono text-sm rounded-lg group bg-pollen text-bark">
                                <div><span class="text-comb">"$ "</span>"git add src/feature.rs"</div>
                                <div><span class="text-comb">"$ "</span>"commitbee"</div>
                                <button class="absolute text-xs transition-opacity opacity-0 copy-btn top-2 right-2 group-hover:opacity-100 focus:opacity-100 text-comb hover:text-honey" data-code="git add src/feature.rs && commitbee">"Copy"</button>
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
