// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="relative border-t border-honey/10 bg-surface-raised">
            <div class="absolute inset-0 hex-bg pointer-events-none"/>

            <div class="relative mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8">
                <div class="grid grid-cols-1 gap-8 md:grid-cols-3">
                    // Brand
                    <div>
                        <div class="flex items-center gap-2 font-bold text-lg text-bark">
                            <span class="text-2xl">"🐝"</span>
                            <span>"CommitBee"</span>
                        </div>
                        <p class="mt-2 text-sm text-comb">
                            "The commit message generator that actually understands your code."
                        </p>
                    </div>

                    // Links
                    <div>
                        <h3 class="text-sm font-semibold text-bark">"Links"</h3>
                        <ul class="mt-3 space-y-2">
                            <li>
                                <a href="/docs/getting-started" class="text-sm text-comb hover:text-honey transition-colors">"Documentation"</a>
                            </li>
                            <li>
                                <a href="https://github.com/sephyi/commitbee" target="_blank" rel="noopener noreferrer" class="text-sm text-comb hover:text-honey transition-colors">"GitHub"</a>
                            </li>
                            <li>
                                <a href="https://crates.io/crates/commitbee" target="_blank" rel="noopener noreferrer" class="text-sm text-comb hover:text-honey transition-colors">"crates.io"</a>
                            </li>
                        </ul>
                    </div>

                    // Meta
                    <div>
                        <h3 class="text-sm font-semibold text-bark">"Project"</h3>
                        <ul class="mt-3 space-y-2">
                            <li>
                                <a href="https://github.com/sponsors/Sephyi" target="_blank" rel="noopener noreferrer" class="text-sm text-comb hover:text-honey transition-colors">"Sponsor"</a>
                            </li>
                            <li>
                                <span class="text-sm text-comb">"License: PolyForm Noncommercial"</span>
                            </li>
                        </ul>
                    </div>
                </div>

                <div class="mt-8 border-t border-honey/10 pt-6 text-center">
                    <p class="text-xs text-comb">
                        "Made with Rust" " · "
                        "Copyright 2026 " <a href="https://sephy.io" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"Sephyi"</a>
                    </p>
                </div>
            </div>
        </footer>
    }
}
