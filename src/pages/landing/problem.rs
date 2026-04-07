// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn ProblemSection() -> impl IntoView {
    view! {
        <section class="py-24 bg-surface-raised">
            <div class="mx-auto max-w-6xl px-4">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "Every other tool just pipes your diff to an LLM and "
                        <span class="text-honey">"hopes for the best."</span>
                    </h2>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-16 grid grid-cols-1 gap-8 md:grid-cols-2">
                    <crate::components::scroll_reveal::ScrollReveal class="reveal-left">
                        <div class="rounded-xl border border-red-500/20 bg-surface p-6">
                            <h3 class="text-sm font-semibold text-red-500 mb-4">"What other tools see"</h3>
                            <pre class="font-mono text-xs text-comb overflow-x-auto">
                                <code>
                                    "- fn validate(&self) -> bool {\n"
                                    "-     self.token.len() > 0\n"
                                    "- }\n"
                                    "+ fn validate(&self) -> Result<(), AuthError> {\n"
                                    "+     if self.token.is_empty() {\n"
                                    "+         return Err(AuthError::EmptyToken);\n"
                                    "+     }\n"
                                    "+     self.check_expiry()?;\n"
                                    "+     Ok(())\n"
                                    "+ }"
                                </code>
                            </pre>
                            <div class="mt-4 rounded-lg bg-red-500/5 p-3">
                                <p class="text-sm text-comb italic">"\"update validate function\""</p>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>

                    <crate::components::scroll_reveal::ScrollReveal class="reveal-right">
                        <div class="rounded-xl border border-honey/30 bg-surface p-6">
                            <h3 class="text-sm font-semibold text-honey mb-4">"What CommitBee sees"</h3>
                            <div class="space-y-3">
                                <div class="rounded-lg bg-pollen p-3">
                                    <p class="text-xs font-semibold text-honey">"Symbol: validate()"</p>
                                    <p class="text-xs text-comb">"Modified signature: bool -> Result<(), AuthError>"</p>
                                </div>
                                <div class="rounded-lg bg-pollen p-3">
                                    <p class="text-xs font-semibold text-honey">"Evidence: BREAKING_CHANGE"</p>
                                    <p class="text-xs text-comb">"Public API return type changed"</p>
                                </div>
                                <div class="rounded-lg bg-pollen p-3">
                                    <p class="text-xs font-semibold text-honey">"Evidence: BUG_FIX"</p>
                                    <p class="text-xs text-comb">"Error handling added (was ignoring failures)"</p>
                                </div>
                            </div>
                            <div class="mt-4 rounded-lg bg-honey/10 p-3 border border-honey/20">
                                <p class="text-sm text-bark font-medium">"\"fix(auth)!: return Result from validate with expiry check\""</p>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>
                </div>
            </div>
        </section>
    }
}
