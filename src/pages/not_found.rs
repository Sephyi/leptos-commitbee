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
                <div class="text-6xl mb-4">"🐝"</div>
                <h1 class="text-5xl font-bold text-bark">"404"</h1>
                <p class="mt-4 text-lg text-comb">"This page buzzed off somewhere."</p>
                <div class="mt-8 flex items-center justify-center gap-4">
                    <a href="/" class="rounded-lg bg-honey px-6 py-3 text-sm font-semibold text-white hover:bg-honey-dark transition-colors">
                        "Go Home"
                    </a>
                    <a href="/docs/getting-started" class="rounded-lg border border-honey/30 px-6 py-3 text-sm font-semibold text-bark hover:bg-honey/5 transition-colors">
                        "Read Docs"
                    </a>
                </div>
            </div>
        </div>
    }
}
