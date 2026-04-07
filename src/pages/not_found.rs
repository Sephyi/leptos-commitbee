// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text="404 - CommitBee"/>
        <div class="flex items-center justify-center min-h-screen">
            <div class="text-center">
                <div class="mb-4 text-6xl">"🐝"</div>
                <h1 class="text-5xl font-bold text-bark">"404"</h1>
                <p class="mt-4 text-lg text-comb">"This page buzzed off somewhere."</p>
                <div class="flex items-center justify-center gap-4 mt-8">
                    <a href="/" class="px-6 py-3 text-sm font-semibold text-white transition-colors rounded-lg bg-honey hover:bg-honey-dark">
                        "Go Home"
                    </a>
                    <a href="/docs/getting-started" class="px-6 py-3 text-sm font-semibold transition-colors border rounded-lg border-honey/30 text-bark hover:bg-honey/5">
                        "Read Docs"
                    </a>
                </div>
            </div>
        </div>
    }
}
