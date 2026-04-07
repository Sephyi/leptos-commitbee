// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <Title text="CommitBee - The commit message generator that actually understands your code"/>
        <Meta name="description" content="CommitBee uses tree-sitter semantic analysis and LLMs to generate high-quality conventional commit messages. Rust CLI tool."/>
        <Meta property="og:title" content="CommitBee"/>
        <Meta property="og:description" content="The commit message generator that actually understands your code."/>
        <Meta property="og:type" content="website"/>
        <Link rel="canonical" href="https://commitbee.dev/"/>

        <main id="main-content">
            <HeroSection/>
            <ProblemSection/>
            <PipelineSection/>
            <DifferentiatorsSection/>
            <ComparisonSection/>
            <InstallSection/>
            <DocsTransitionSection/>
            <crate::components::code_block::CodeBlockActivator/>
        </main>
    }
}

// --- Section 1: Hero ---

#[component]
fn HeroSection() -> impl IntoView {
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

// --- Section 2: Problem Statement ---

#[component]
fn ProblemSection() -> impl IntoView {
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

// --- Section 3: Pipeline Demo ---

#[component]
fn PipelineSection() -> impl IntoView {
    view! {
        <section id="pipeline" class="py-24">
            <div class="mx-auto max-w-5xl px-4">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "A " <span class="text-honey">"7-stage pipeline"</span> " from diff to commit"
                    </h2>
                    <p class="mt-4 text-center text-comb max-w-2xl mx-auto">
                        "Watch how CommitBee processes your code, step by step."
                    </p>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-16">
                    <crate::components::pipeline_demo::PipelineDemo/>
                </div>
            </div>
        </section>
    }
}

// --- Section 4: Differentiators ---

#[component]
fn DifferentiatorsSection() -> impl IntoView {
    view! {
        <section class="py-24 bg-surface-raised">
            <div class="mx-auto max-w-6xl px-4">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "What sets CommitBee " <span class="text-honey">"apart"</span>
                    </h2>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-16 grid grid-cols-1 gap-8 md:grid-cols-3 reveal-stagger">
                    <FeatureCard
                        title="Tree-sitter Semantic Analysis"
                        description="It reads your code, not just your diffs. Extracts 10 symbol types across 10 languages and maps diff hunks to their spans."
                        icon="🌳"
                    />
                    <FeatureCard
                        title="Commit Splitting"
                        description="It detects mixed concerns and splits them into separate, well-typed commits using diff-shape fingerprinting."
                        icon="🔀"
                    />
                    <FeatureCard
                        title="25-Pattern Secret Scanning"
                        description="It catches leaked credentials before they reach any LLM. 25 patterns across 13 categories, fully customizable."
                        icon="🔒"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(
    title: &'static str,
    description: &'static str,
    icon: &'static str,
) -> impl IntoView {
    view! {
        <crate::components::scroll_reveal::ScrollReveal>
            <div class="rounded-xl border border-honey/20 bg-surface p-6 hover:border-honey/40 hover:shadow-lg hover:shadow-honey/5 transition-all">
                <div class="text-3xl mb-4">{icon}</div>
                <h3 class="text-lg font-semibold text-bark">{title}</h3>
                <p class="mt-2 text-sm text-comb">{description}</p>
            </div>
        </crate::components::scroll_reveal::ScrollReveal>
    }
}

// --- Section 5: Competitive Comparison ---

#[component]
fn ComparisonSection() -> impl IntoView {
    view! {
        <section class="py-24">
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

// --- Section 6: Install + Quick Start ---

#[component]
fn InstallSection() -> impl IntoView {
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

// --- Section 7: Docs Transition ---

#[component]
fn DocsTransitionSection() -> impl IntoView {
    view! {
        <section class="py-24">
            <div class="mx-auto max-w-5xl px-4">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "Dive deeper"
                    </h2>
                    <p class="mt-4 text-center text-comb max-w-xl mx-auto">
                        "Explore the full documentation to master CommitBee."
                    </p>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-12 grid grid-cols-1 gap-4 sm:grid-cols-2">
                    <DocLink slug="getting-started" title="Getting Started" description="Install and generate your first commit message"/>
                    <DocLink slug="configuration" title="Configuration" description="5-level config system, TOML files, environment variables"/>
                    <DocLink slug="llm-providers" title="LLM Providers" description="Ollama, OpenAI, and Anthropic setup"/>
                    <DocLink slug="architecture" title="Architecture" description="Deep dive into the pipeline internals"/>
                </div>
            </div>
        </section>
    }
}

#[component]
fn DocLink(slug: &'static str, title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <crate::components::scroll_reveal::ScrollReveal>
            <a
                href=format!("/docs/{slug}")
                class="block rounded-xl border border-honey/20 bg-surface-raised p-6 hover:border-honey/40 hover:shadow-lg hover:shadow-honey/5 transition-all group"
            >
                <h3 class="text-lg font-semibold text-bark group-hover:text-honey transition-colors">{title}</h3>
                <p class="mt-1 text-sm text-comb">{description}</p>
                <span class="mt-3 inline-block text-sm text-honey">"Read more →"</span>
            </a>
        </crate::components::scroll_reveal::ScrollReveal>
    }
}
