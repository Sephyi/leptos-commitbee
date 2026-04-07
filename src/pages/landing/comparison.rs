// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn ComparisonSection() -> impl IntoView {
    view! {
        <section class="py-24 bg-surface/55 backdrop-blur-md">
            <div class="mx-auto max-w-5xl px-4">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "CommitBee vs. the field"
                    </h2>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-12 relative">
                    <crate::components::scroll_reveal::ScrollReveal class="reveal-scale">
                        <div class="rounded-xl border border-comb/10 bg-surface-raised p-6 ml-4 mr-0 sm:ml-8 sm:mr-0">
                            <h3 class="text-sm font-semibold text-comb mb-4">"Other commit generators"</h3>
                            <div class="grid grid-cols-2 gap-3 sm:grid-cols-3">
                                <ComparisonFeature label="Local LLM" has=true/>
                                <ComparisonFeature label="Cloud providers" has=true/>
                                <ComparisonFeature label="Git hooks" has=true/>
                                <ComparisonFeature label="Streaming output" has=true/>
                                <ComparisonFeature label="Tree-sitter AST" has=false/>
                                <ComparisonFeature label="Commit splitting" has=false/>
                                <ComparisonFeature label="Secret scanning" has=false/>
                                <ComparisonFeature label="Evidence typing" has=false/>
                                <ComparisonFeature label="Multi-pass validation" has=false/>
                                <ComparisonFeature label="Token budget" has=false/>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>

                    <crate::components::scroll_reveal::ScrollReveal class="reveal-scale">
                        <div class="-mt-8 relative z-10 rounded-xl border-2 border-honey/30 bg-surface p-6 shadow-xl shadow-honey/10 mr-4 ml-0 sm:mr-8 sm:ml-0">
                            <div class="flex items-center gap-2 mb-4">
                                <span class="text-xl">"🐝"</span>
                                <h3 class="text-sm font-semibold text-honey">"CommitBee"</h3>
                            </div>
                            <div class="grid grid-cols-2 gap-3 sm:grid-cols-3">
                                <ComparisonFeature label="Tree-sitter AST" has=true/>
                                <ComparisonFeature label="Commit splitting" has=true/>
                                <ComparisonFeature label="Secret scanning" has=true/>
                                <ComparisonFeature label="Evidence typing" has=true/>
                                <ComparisonFeature label="Multi-pass validation" has=true/>
                                <ComparisonFeature label="Token budget" has=true/>
                                <ComparisonFeature label="Local LLM" has=true/>
                                <ComparisonFeature label="Cloud providers" has=true/>
                                <ComparisonFeature label="Git hooks" has=true/>
                                <ComparisonFeature label="Streaming output" has=true/>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ComparisonFeature(label: &'static str, has: bool) -> impl IntoView {
    view! {
        <div class=format!(
            "flex items-center gap-2 rounded-lg px-3 py-2 text-sm {}",
            if has { "bg-honey/5 text-bark" } else { "bg-surface-raised text-comb/50" }
        )>
            {if has {
                view! { <span class="text-honey font-medium">"✓"</span> }.into_any()
            } else {
                view! { <span class="text-comb/30">"—"</span> }.into_any()
            }}
            <span>{label}</span>
        </div>
    }
}
