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
use leptos_meta::*;

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
