// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

mod comparison;
mod differentiators;
mod docs_transition;
mod hero;
mod install;
mod pipeline;
mod problem;

use leptos::prelude::*;

use comparison::ComparisonSection;
use differentiators::DifferentiatorsSection;
use docs_transition::DocsTransitionSection;
use hero::HeroSection;
use install::InstallSection;
use pipeline::PipelineSection;
use problem::ProblemSection;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <crate::components::seo::SeoMeta
            title="CommitBee – Semantic AI Commit Message Generator"
            description="CommitBee is a Rust CLI that generates Conventional Commit messages from your staged changes using tree-sitter semantic analysis and local or cloud LLMs."
            path="/"
        />

        <main id="main-content">
            <HeroSection/>
            <ProblemSection/>
            <PipelineSection/>
            // TODO: <DifferentiatorsSection/>
            // TODO: <ComparisonSection/>
            // TODO: <InstallSection/>
            <DocsTransitionSection/>
            <crate::components::code_block::CodeBlockActivator/>
        </main>
    }
}
