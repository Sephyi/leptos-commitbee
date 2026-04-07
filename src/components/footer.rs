// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

/// Renders three candidate footer variants stacked vertically for A/B/D review.
/// Once a variant is chosen, delete the other two and inline the winner here.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <FooterVariantA/>
        <FooterVariantB/>
        <FooterVariantD/>
    }
}

// ---------------------------------------------------------------------------
// Variant A: minimal single-line (Vercel / Linear style)
// ---------------------------------------------------------------------------
#[component]
fn FooterVariantA() -> impl IntoView {
    view! {
        <footer class="border-t border-honey/10 bg-surface/55 backdrop-blur-md">
            <div class="w-full px-4 py-6">
                // Variant label (remove once chosen)
                <div class="mb-3 text-center text-[10px] font-semibold uppercase tracking-widest text-honey/60">
                    "Variant A · Minimal single-line"
                </div>
                <div class="flex flex-wrap items-center justify-center gap-x-4 gap-y-2 text-sm text-comb">
                    <span class="flex items-center gap-1.5 font-semibold text-bark">
                        <span class="text-lg">"🐝"</span>
                        "CommitBee"
                    </span>
                    <span class="text-comb/40">"·"</span>
                    <a href="/docs/getting-started" class="hover:text-honey transition-colors">"Docs"</a>
                    <a href="https://github.com/sephyi/commitbee" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"GitHub"</a>
                    <a href="https://crates.io/crates/commitbee" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"crates.io"</a>
                    <a href="https://github.com/sponsors/Sephyi" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"Sponsor"</a>
                    <span class="text-comb/40">"·"</span>
                    <a href="/imprint" class="hover:text-honey transition-colors">"Imprint"</a>
                    <a href="/privacy" class="hover:text-honey transition-colors">"Privacy"</a>
                    <span class="text-comb/40">"·"</span>
                    <span>"© 2026 "<a href="https://sephy.io" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"Sephyi"</a></span>
                </div>
            </div>
        </footer>
    }
}

// ---------------------------------------------------------------------------
// Variant B: centered wordmark + inline link row (cinematic close)
// ---------------------------------------------------------------------------
#[component]
fn FooterVariantB() -> impl IntoView {
    view! {
        <footer class="border-t border-honey/10 bg-surface/55 backdrop-blur-md">
            <div class="w-full px-4 py-16 text-center">
                // Variant label (remove once chosen)
                <div class="mb-6 text-[10px] font-semibold uppercase tracking-widest text-honey/60">
                    "Variant B · Centered wordmark"
                </div>
                <div class="flex items-center justify-center gap-3">
                    <span class="text-4xl">"🐝"</span>
                    <span class="text-3xl font-bold text-bark">"CommitBee"</span>
                </div>
                <p class="mt-3 mx-auto max-w-md text-sm text-comb">
                    "The commit message generator that actually understands your code."
                </p>
                <nav class="mt-8 flex flex-wrap items-center justify-center gap-x-5 gap-y-2 text-sm text-comb">
                    <a href="/docs/getting-started" class="hover:text-honey transition-colors">"Docs"</a>
                    <span class="text-comb/30">"·"</span>
                    <a href="https://github.com/sephyi/commitbee" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"GitHub"</a>
                    <span class="text-comb/30">"·"</span>
                    <a href="https://crates.io/crates/commitbee" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"crates.io"</a>
                    <span class="text-comb/30">"·"</span>
                    <a href="https://github.com/sponsors/Sephyi" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"Sponsor"</a>
                    <span class="text-comb/30">"·"</span>
                    <a href="/imprint" class="hover:text-honey transition-colors">"Imprint"</a>
                    <span class="text-comb/30">"·"</span>
                    <a href="/privacy" class="hover:text-honey transition-colors">"Privacy"</a>
                </nav>
                <p class="mt-8 text-xs text-comb">
                    "Made with Rust · © 2026 "
                    <a href="https://sephy.io" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"Sephyi"</a>
                </p>
            </div>
        </footer>
    }
}

// ---------------------------------------------------------------------------
// Variant D: marquee wordmark + compact legal strip
// ---------------------------------------------------------------------------
#[component]
fn FooterVariantD() -> impl IntoView {
    view! {
        <footer class="border-t border-honey/10 bg-surface/55 backdrop-blur-md">
            // Variant label (remove once chosen)
            <div class="pt-4 text-center text-[10px] font-semibold uppercase tracking-widest text-honey/60">
                "Variant D · Marquee wordmark"
            </div>
            // Giant faded wordmark as visual anchor
            <div class="relative overflow-hidden px-4 pt-8 pb-4">
                <h2 class="select-none text-center font-black text-honey/10 leading-none tracking-tight text-[clamp(4rem,18vw,14rem)]">
                    "commitbee"
                </h2>
            </div>
            // Thin compact strip
            <div class="border-t border-honey/10 w-full px-4 py-4">
                <div class="flex flex-wrap items-center justify-between gap-x-4 gap-y-2 text-xs text-comb">
                    <span class="flex items-center gap-1.5">
                        <span class="text-base">"🐝"</span>
                        "© 2026 "
                        <a href="https://sephy.io" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"Sephyi"</a>
                    </span>
                    <nav class="flex flex-wrap items-center gap-x-4 gap-y-1">
                        <a href="/docs/getting-started" class="hover:text-honey transition-colors">"Docs"</a>
                        <a href="https://github.com/sephyi/commitbee" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"GitHub"</a>
                        <a href="https://github.com/sponsors/Sephyi" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"Sponsor"</a>
                        <a href="/imprint" class="hover:text-honey transition-colors">"Imprint"</a>
                        <a href="/privacy" class="hover:text-honey transition-colors">"Privacy"</a>
                    </nav>
                </div>
            </div>
        </footer>
    }
}
