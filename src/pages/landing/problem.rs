// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn ProblemSection() -> impl IntoView {
    view! {
        <section class="py-24 bg-surface/55 backdrop-blur-md [mask-image:linear-gradient(to_bottom,transparent_0,black_140px)] [-webkit-mask-image:linear-gradient(to_bottom,transparent_0,black_140px)]">
            <div class="max-w-6xl px-4 mx-auto">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "Every other tool just pipes your diff to an LLM and "
                        <span class="text-honey">"hopes for the best."</span>
                    </h2>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="grid grid-cols-1 gap-8 mt-16 md:grid-cols-2">
                    <crate::components::scroll_reveal::ScrollReveal class="reveal-left">
                        <div class="p-6 border rounded-xl border-red-500/20 bg-surface">
                            <h3 class="mb-4 text-sm font-semibold text-red-500">"What other tools see"</h3>
                            <pre class="overflow-x-auto font-mono text-xs text-comb">
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
                            <div class="p-3 mt-4 rounded-lg bg-red-500/5">
                                <p class="text-sm italic text-comb">"\"update validate function\""</p>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>

                    <crate::components::scroll_reveal::ScrollReveal class="reveal-right">
                        <div class="p-6 border rounded-xl border-honey/30 bg-surface">
                            <h3 class="mb-4 text-sm font-semibold text-honey">"What CommitBee sees"</h3>
                            <div class="space-y-3">
                                <div class="p-3 rounded-lg bg-pollen">
                                    <p class="text-xs font-semibold text-honey">"Symbol: validate()"</p>
                                    <p class="text-xs text-comb">"Modified signature: bool -> Result<(), AuthError>"</p>
                                </div>
                                <div class="p-3 rounded-lg bg-pollen">
                                    <p class="text-xs font-semibold text-honey">"Evidence: BREAKING_CHANGE"</p>
                                    <p class="text-xs text-comb">"Public API return type changed"</p>
                                </div>
                                <div class="p-3 rounded-lg bg-pollen">
                                    <p class="text-xs font-semibold text-honey">"Evidence: BUG_FIX"</p>
                                    <p class="text-xs text-comb">"Error handling added (was ignoring failures)"</p>
                                </div>
                            </div>
                            <div class="p-3 mt-4 border rounded-lg bg-honey/10 border-honey/20">
                                <p class="text-sm font-medium text-bark">"\"fix(auth)!: return Result from validate with expiry check\""</p>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>
                </div>
            </div>
        </section>
    }
}
