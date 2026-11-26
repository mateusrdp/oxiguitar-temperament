#!/usr/bin/env python3
"""Generate batch-*.json files from batches.json and the project files."""

import json
import os

PROJECT_ROOT = "/home/mat/Documents/src/oxiguitar-temperament/.worktrees/update-ua"
BATCHES_FILE = os.path.join(PROJECT_ROOT, ".ua", "intermediate", "batches.json")
OUTPUT_DIR = os.path.join(PROJECT_ROOT, ".ua", "intermediate")

with open(BATCHES_FILE) as f:
    batch_data = json.load(f)

# ------------------------------------------------------------------- helpers

def node(file_path, node_type, name, summary, tags=None, complexity="low"):
    return {
        "id": f"{node_type}:{file_path}",
        "type": node_type,
        "name": name,
        "summary": summary,
        "tags": tags or [],
        "complexity": complexity,
        "filePath": file_path,
    }

def edge(source, target, relation):
    return {"source": source, "target": target, "type": relation}

# ================================================================ batch 1: infra

def make_batch1():
    files = []
    edges = []

    files.append(node(
        ".github/workflows/ci.yml", "file", "CI workflow",
        "Main CI pipeline: formatting, linting (clippy on MSRV), tests on 6 platforms (Linux x86_64/ARM64, Windows x86_64/ARM64, macOS), dependency/license audit (cargo-deny), DCO sign-off check, and a ci-success aggregator job. Uses scripts/apt-install.sh for resilient package installation on slow mirrors.",
        tags=["ci", "workflow", "github-actions", "testing", "linting"],
        complexity="medium",
    ))
    files.append(node(
        ".github/workflows/docs.yml", "file", "Docs workflow",
        "Builds the mdBook manual on pushes to main and PRs touching docs/manual/. Installs a pinned mdBook binary, builds the manual, checks internal links for broken references, and deploys to GitHub Pages via actions/deploy-pages.",
        tags=["ci", "workflow", "github-actions", "docs", "deployment"],
        complexity="low",
    ))
    files.append(node(
        ".github/workflows/publish.yml", "file", "Publish workflow",
        "Triggers on release published events or manual dispatch. Delegates to oxiguitar-packages' reusable plugin-publish.yml workflow to notify package repositories (Debian/Ubuntu, Fedora, Flatpak, winget) when a new release is available.",
        tags=["ci", "workflow", "github-actions", "publishing"],
        complexity="low",
    ))
    files.append(node(
        ".github/workflows/release.yml", "file", "Release workflow",
        "Runs oxiguitar-packages' reusable plugin-release.yml on PRs, pushes to develop/main, and v* tags. Builds plugin files for 6 targets (Linux x86_64/ARM64, Windows x86_64/ARM64, macOS universal) via build-oxp.sh, producing zips, .deb, .rpm, Windows installer, and macOS .pkg.",
        tags=["ci", "workflow", "github-actions", "release"],
        complexity="low",
    ))

    return files, edges

# ================================================================ batch 2: config + root docs

def make_batch2():
    files = []
    edges = []

    files.append(node(
        "CHANGELOG.md", "file", "Changelog",
        "Keeps a Changelog format changelog tracking all notable changes. Covers versions 0.1.0 (initial release with three temperament modes), 0.2.0 (tar archive plugin format, test-all.py runner, improved manual), and 0.3.0 (Slint UI integration, plugin interface 1.16).",
        tags=["docs", "changelog", "versioning"],
        complexity="low",
    ))
    files.append(node(
        "CLAUDE.md", "file", "Claude working instructions",
        "Standing rules for working on this repository with Claude: library usage policy, no telemetry/uploads, no emojis/signatures, branch strategy (develop/main), rust-version management, DCO sign-off rules, test-all.py requirements, and PR workflow via the REST API.",
        tags=["docs", "claude", "procedural"],
        complexity="low",
    ))
    files.append(node(
        "CONTRIBUTING.md", "file", "Contributing guide",
        "Contribution guidelines for the repository: branch strategy (develop as default, main for releases), squash-merge workflow, CI requirements, test expectations (every feature needs a row on the Features page), and DCO sign-off process with special handling for bot commits.",
        tags=["docs", "contributing", "procedural"],
        complexity="low",
    ))
    files.append(node(
        "Cargo.toml", "file", "Cargo manifest",
        "Rust package manifest for oxiguitar-temperament-plugin v0.3.0, edition 2024, MSRV 1.99. Single dependency on oxiguitar-plugin v0.23.0 (git tag). Dev dependencies on oxiguitar-plugin-test and oxiguitar-host for the contract suite. Configures lints (deny unsafe, warn missing_docs), release profile (thin LTO, codegen-units=1, strip symbols), and an empty workspace.",
        tags=["config", "rust", "dependencies", "build"],
        complexity="low",
    ))
    files.append(node(
        "README.md", "file", "Project README",
        "Project overview describing the temperament plugin: three modes (Off, Realistic guitar, True Temperament: Thidell Formula 1), per-track settings saved in .oxg sidecar files, pitch curve modification without score changes. Includes manual link, install instructions, build/test commands, credits, and GPL-3.0-or-later license.",
        tags=["docs", "readme", "overview"],
        complexity="low",
    ))
    files.append(node(
        "build-oxp.sh", "file", "Plugin build script",
        "Builds the plugin .oxp file for the current system: cross-compiles for macOS universal (lipo), builds .dll for Windows and .so for Linux. Packs manifest.json and the dynamic library into a tar archive (plugin.so/dll/dylib). Output goes to dist/ with naming convention temperament-VERSION-SYSTEM-ARCH.oxp.",
        tags=["script", "build", "release"],
        complexity="low",
    ))
    files.append(node(
        "clippy.toml", "file", "Clippy configuration",
        "Clippy linter settings: MSRV 1.99, allows unwrap/expect/panic/indexing-slicing in tests only.",
        tags=["config", "linting", "clippy"],
        complexity="low",
    ))
    files.append(node(
        "deny.toml", "file", "Cargo deny configuration",
        "cargo-deny configuration for license compliance and security: targets all 6 build targets, allows GPL-3.0, MIT, Apache-2.0, BSD, ISC, Zlib, Unicode, BSL, CC0, OFL licenses, denies unknown registries and git sources, warns on multiple versions, denies wildcards. Only allows crates.io and mateusrdp/oxiguitar git.",
        tags=["config", "security", "licensing"],
        complexity="low",
    ))
    files.append(node(
        "manifest.json", "file", "Plugin manifest",
        "OxiGuitar plugin manifest declaring interface 1.16, id oxiguitar.temperament, name Temperament v0.3.0. Declares performs capability, a track section (temperament, plays on fretted instruments), a Slint UI component (TemperamentSection), and the UI view placement in the track tab.",
        tags=["config", "plugin", "manifest"],
        complexity="low",
    ))
    files.append(node(
        "rustfmt.toml", "file", "Rust formatter configuration",
        "Rustfmt settings: edition 2024, max width 100, uses field init shorthand.",
        tags=["config", "formatting", "rustfmt"],
        complexity="low",
    ))

    return files, edges

# ================================================================ batch 3: manual docs

def make_batch3():
    files = []
    edges = []

    files.append(node(
        "docs/manual/src/README.md", "file", "Manual home page",
        "Main manual page explaining the temperament plugin's purpose: why real guitars don't play exact pitches (stretching, stiffness), what cents are, pure intervals vs equal temperament, why equal temperament works, guitar intonation errors, Perfect temperament as a future direction, True Temperament/Thidell Formula 1 explanation, notable users, and a summary table of the three modes.",
        tags=["docs", "manual", "overview", "music-theory"],
        complexity="medium",
    ))
    files.append(node(
        "docs/manual/src/SUMMARY.md", "file", "Manual table of contents",
        "mdBook summary file listing all manual sections: home, how to use it, architecture and strategy, requirements, installation, features and tests, testing guide, and credits.",
        tags=["docs", "manual", "navigation"],
        complexity="low",
    ))
    files.append(node(
        "docs/manual/src/credits.md", "file", "Credits page",
        "Acknowledgments page listing code dependencies (OxiGuitar SDK, Rust), research sources (Varieschi & Gower, Anderson & Beausoleil, Thidell Formula 1, D'Addario tension charts), tools (mdBook, cargo-deny), and thanks to Anders Thidell, Paul Guy, and the physics community.",
        tags=["docs", "manual", "credits"],
        complexity="low",
    ))
    files.append(node(
        "docs/manual/src/features.md", "file", "Features page",
        "Features table mapping each plugin feature to its tests: Off by default, Realistic guitar, Four guitars or track's own, True Temperament Thidell Formula 1, bends/slides/vibrato shape preservation, open strings/harmonics/dead notes handling, fretted tracks only, the Temperament UI section, through OxiGuitar's player, and a working plugin file. CI validates every feature names at least one existing test.",
        tags=["docs", "manual", "features", "testing"],
        complexity="low",
    ))
    files.append(node(
        "docs/manual/src/how-it-works.md", "file", "Architecture and strategy",
        "Technical architecture document: how the plugin fits as a performer in OxiGuitar, the realistic guitar model (straight-line string segments, saddle compensation, two published papers), True Temperament implementation, current limitations (one finger pressure, slides ignore passing frets, whole cents only, four presets, no nut compensation, frets above 24th, bass uses same table), what's left out on purpose, and where community help is wanted.",
        tags=["docs", "manual", "architecture", "design"],
        complexity="medium",
    ))
    files.append(node(
        "docs/manual/src/install.md", "file", "Installation guide",
        "Installation instructions: release downloads (zip per system, .deb, .rpm, Flatpak, Windows installer, macOS .pkg), manual copy instructions with platform-specific plugin folders, macOS quarantine removal, Linux package manager setup (Debian/Ubuntu APT, Fedora DNF), and Flatpak extension configuration.",
        tags=["docs", "manual", "installation"],
        complexity="low",
    ))
    files.append(node(
        "docs/manual/src/requirements.md", "file", "Requirements and assumptions",
        "System requirements: OxiGuitar 0.23.0+ (plugin interface 1.16), a sound plugin playing each string on its own pitch (MIDI plugin), Windows/macOS/Linux support. Assumptions: guitar set up with 12th fret exact octave, whole cents are sufficient, slides ignore passing frets.",
        tags=["docs", "manual", "requirements"],
        complexity="low",
    ))
    files.append(node(
        "docs/manual/src/testing.md", "file", "Testing guide",
        "Testing documentation: running all tests with scripts/test-all.py, required tools (Rust, git, Python 3.8+, Linux build libraries), the three test stages (cargo test, plugin on newest OxiGuitar release, Features page validation), output files (summary.md, test-all.log), failure reporting procedure, and CLI options (--release, --ignore-rust-version, --core, --out).",
        tags=["docs", "manual", "testing"],
        complexity="low",
    ))
    files.append(node(
        "docs/manual/src/use-cases.md", "file", "How to use it",
        "User guide: picking a temperament for a track via the Track tab's Temperament section, hearing a real guitar's intonation (Realistic guitar mode with four guitar types), hearing True Temperament frets (Thidell Formula 1), going back to normal (Off mode), and tips (few cents effect, bends/slides keep shape, capo counts as fret, audio export support).",
        tags=["docs", "manual", "usage", "user-guide"],
        complexity="low",
    ))

    # Internal doc links
    for page in ["use-cases.md", "how-it-works.md", "requirements.md", "install.md",
                 "features.md", "testing.md", "credits.md"]:
        edges.append(edge("file:docs/manual/src/README.md", f"file:docs/manual/src/{page}", "references"))

    edges.append(edge("file:docs/manual/src/SUMMARY.md", "file:docs/manual/src/README.md", "contains"))
    for page in ["use-cases.md", "how-it-works.md", "requirements.md", "install.md",
                 "features.md", "testing.md", "credits.md"]:
        edges.append(edge("file:docs/manual/src/SUMMARY.md", f"file:docs/manual/src/{page}", "references"))

    return files, edges

# ================================================================ batch 4: code + remaining

def make_batch4():
    files = []
    edges = []

    # --- Infra ---
    files.append(node(
        ".github/dependabot.yml", "file", "Dependabot configuration",
        "Weekly version updates for Rust crates and GitHub Actions workflows, targeting the develop branch. Limits to 5 open PRs. Ignores oxiguitar-* dependencies since they come from OxiGuitar's git tags and Dependabot cannot resolve cross-tag version mismatches.",
        tags=["config", "dependabot", "dependency-management"],
        complexity="low",
    ))
    files.append(node(
        ".github/pull_request_template.md", "file", "PR template",
        "GitHub pull request template requiring: description of changes with issue link, testing instructions, and a checklist (tests, docs/CHANGELOG/ROADMAP, CREDITS, no corpus files, no network use, DCO sign-off).",
        tags=["config", "github", "procedural"],
        complexity="low",
    ))
    files.append(node(
        "docs/design.md", "file", "Design document",
        "Original design specification from issue #294: plugin architecture (per-track settings, Track tab Slint section, Perform interface 1.12), how it plays (only fretted tracks, note offset calculation, bend/slide handling, dead notes/harmonics), realistic guitar model (stretching, stiffness formulas, four guitar presets with D'Addario strings, finger press parameter), True Temperament implementation (Thidell Formula 1 offsets), test coverage, and future work (Perfect Temperament, slide fret-following, custom setups).",
        tags=["docs", "design", "specification"],
        complexity="high",
    ))
    files.append(node(
        "docs/manual/book.toml", "file", "mdBook configuration",
        "mdBook settings for the Temperament plugin manual: title, authors, English language, source directory src. Configures HTML output with GitHub repository URL, edit URL template, light/navy themes, no section labels, and search enabled.",
        tags=["config", "docs", "mdbook"],
        complexity="low",
    ))
    files.append(node(
        "scripts/apt-install.sh", "file", "APT install script",
        "Resilient apt package installation for CI: monitors download progress to detect stalled mirrors, rotates mirrors on failure (up to 5 tries), keeps downloaded packages in cache, stops only when downloads are stuck (not slow). Uses Python to reorder mirror priorities. Shared across all OxiGuitar repositories. Environment variables: APT_STALL_SECONDS (120), APT_TRIES (5).",
        tags=["script", "ci", "infrastructure"],
        complexity="medium",
    ))
    files.append(node(
        "scripts/check-dco.sh", "file", "DCO sign-off checker",
        "Checks Developer Certificate of Origin sign-off on all commits in a range (BASE..HEAD). Skips bot commits (Claude app via noreply@anthropic.com, Dependabot via noreply@github.com with Co-Authored-By). Verifies Signed-off-by trailer matches author email. Used in CI's dco job on pull requests.",
        tags=["script", "ci", "dco", "procedural"],
        complexity="low",
    ))
    files.append(node(
        "scripts/test-all.py", "file", "Test runner",
        "Unified test runner for all OxiGuitar repositories (1393 lines). Builds and runs cargo test, tests plugins against the newest OxiGuitar release, validates the Features page. Features: progress bar, environment detection (host triple, OS, arch), corpus testing through oxiguitar-cli (detect, convert, info), plugin contract checking, plugin file building (tar archives), feature-to-test validation, structured summary output (summary.md, test-all.log, corpus-*.json). Python 3.8+ standard library only.",
        tags=["script", "testing", "runner", "python"],
        complexity="high",
    ))

    # --- Rust source ---
    files.append(node(
        "src/intonation.rs", "struct:src/intonation.rs:StringSpec",
        "Intonation model",
        "Physical model of real guitar intonation errors: StringSpec (MIDI key, tension, core diameter, Young's modulus), Guitar (scale length, nut height, action range, finger press depth, string specs), Tuned (compensated string with saddle setback). Computes fret-to-fret pitch offset from equal temperament due to string stretching and stiffness. Uses Varieschi & Gower (2010) and Anderson & Beausoleil (2024) formulas. Binary search for saddle compensation. FRETS constant caps at 24.",
        tags=["code", "rust", "physics", "intonation", "model"],
        complexity="high",
    ))
    files.append(node(
        "src/lib.rs", "file:src/lib.rs",
        "Plugin library entry point",
        "Root module declaring all public submodules (intonation, perform, section, settings, strings, true_temperament). Defines SECTION constant for the Track tab section ID. TemperamentPlugin struct holding a section::Section, implementing Plugin (start creates the section) and Performer (delegates to perform::perform). Exports the plugin via oxiguitar_plugin::export_plugin!. Includes a test validating manifest.json against the plugin type.",
        tags=["code", "rust", "plugin", "entry-point"],
        complexity="low",
    ))
    files.append(node(
        "src/perform.rs", "file:src/perform.rs",
        "Performance engine",
        "Core performance function: takes a Perform track and returns Vec<PlayedNote> with pitch offsets applied. Handles three modes: Off (no change), Realistic (cents offset from intonation model), Formula1 (Thidell Formula 1 table lookup). Key logic: skips non-fretted tracks, dead notes, and open strings in realistic mode. Harmonics take open string offset. Bends/slides keep their curve shape with the starting fret's offset added. Prepares Tuned setups per staff once, then maps each note.",
        tags=["code", "rust", "performance", "audio"],
        complexity="medium",
    ))
    files.append(node(
        "src/section.rs", "file:src/section.rs",
        "UI section handler",
        "Track tab UI section implementation for the Temperament settings. properties() builds the UI property list (modes, mode, about text, realistic flag, guitars, guitar). change() handles mode-chosen and guitar-chosen callbacks, updating Track settings. Section struct holds a Host and Mutex<Map<String, Json>> for current settings. Implements Ui trait with shown() (pushes properties to the Slint view) and callback() (processes user input, updates track settings via host). Includes a compile-time test validating the Slint file has matching properties and callbacks.",
        tags=["code", "rust", "ui", "slint", "interface"],
        complexity="medium",
    ))
    files.append(node(
        "src/settings.rs", "file:src/settings.rs",
        "Settings persistence",
        "Track temperament settings storage: Mode enum (Off, Realistic, Formula1) with key strings and display names in Mode::ALL array. Track struct holding mode and guitar Kind. from_map/from_text parse JSON settings (graceful fallback to defaults for unknown/invalid input). write() serializes settings into a Map. Mode::key() and Mode::from_key() handle string-to-enum conversion.",
        tags=["code", "rust", "settings", "persistence"],
        complexity="low",
    ))
    files.append(node(
        "src/strings.rs", "file:src/strings.rs",
        "Guitar string definitions",
        "Four guitar presets: NYLON (650mm, D'Addario EJ45), STEEL (25.4in, EJ16), ELECTRIC (25.5in, EXL110), BASS (34in, EXL170). Each defines scale, nut height, action range, press depth, and StringSpec array. Kind enum (Auto, Nylon, Steel, Electric, Bass) with of_program() mapping General MIDI programs to guitar kinds, guitar() returning the appropriate Guitar reference. Constants for unit conversions (LBF, INCH) and material properties (STEEL_E, NYLON_E).",
        tags=["code", "rust", "guitar", "strings", "presets"],
        complexity="medium",
    ))
    files.append(node(
        "src/true_temperament.rs", "file:src/true_temperament.rs",
        "True Temperament model",
        "Thidell Formula 1 temperament implementation: FORMULA_1 array of 12 note offsets (C +2 through B −1 cents), FORMULA_1_OPEN array of 6 standard-tuning open string offsets. formula_1(key) looks up the note's offset by MIDI key modulo 12. formula_1_open(key) returns the published open string tuning for standard strings, falling back to the note offset for other tunings or bass strings.",
        tags=["code", "rust", "temperament", "true-temperament"],
        complexity="low",
    ))

    # --- Tests ---
    files.append(node(
        "tests/contract.rs", "file:tests/contract.rs",
        "Contract test suite",
        "Integration tests that build the plugin as a .oxp file and test it through OxiGuitar's real plugin host. the_plugin_file_keeps_the_contract() validates the built .oxp against the plugin contract checker. the_host_plays_a_track_at_its_temperament() creates test tracks and verifies: Off mode changes nothing, Formula 1 applies correct offsets, Realistic mode sharpens first fret, non-fretted tracks are untouched.",
        tags=["code", "rust", "testing", "integration", "contract"],
        complexity="medium",
    ))
    files.append(node(
        "tests/suite.json", "file:tests/suite.json",
        "Test suite configuration",
        "Test suite metadata declaring the repository name (mateusrdp/oxiguitar-temperament) and plugin configuration (name: temperament, package: oxiguitar-temperament-plugin, library: oxiguitar_temperament_plugin). Used by test-all.py to identify plugins for contract testing.",
        tags=["config", "testing", "suite"],
        complexity="low",
    ))

    # --- UI ---
    files.append(node(
        "ui/temperament.slint", "file:ui/temperament.slint",
        "Temperament UI component",
        "Slint UI definition for the Temperament section in the Track tab. TemperamentSection component with in properties (modes list, mode index, about text, realistic flag, guitars list, guitar index) and callbacks (mode-chosen, guitar-chosen). Layout: Tuning label with mode ComboBox, about text description, conditional Guitar ComboBox shown only when realistic mode is active. Two-way binding between ComboBox index and property.",
        tags=["code", "slint", "ui", "interface"],
        complexity="low",
    ))

    # ============================================================= edges

    # lib.rs -> submodules
    for mod_name in ["intonation", "perform", "section", "settings", "strings", "true_temperament"]:
        edges.append(edge("file:src/lib.rs", f"file:src/{mod_name}.rs", "depends-on"))

    # section.rs dependencies
    edges.append(edge("file:src/section.rs", "file:src/settings.rs", "depends-on"))
    edges.append(edge("file:src/section.rs", "file:src/strings.rs", "depends-on"))

    # perform.rs dependencies
    edges.append(edge("file:src/perform.rs", "file:src/intonation.rs", "depends-on"))
    edges.append(edge("file:src/perform.rs", "file:src/settings.rs", "depends-on"))
    edges.append(edge("file:src/perform.rs", "file:src/true_temperament.rs", "depends-on"))

    # strings.rs -> intonation.rs
    edges.append(edge("file:src/strings.rs", "file:src/intonation.rs", "depends-on"))

    # settings.rs -> strings.rs (Kind)
    edges.append(edge("file:src/settings.rs", "file:src/strings.rs", "depends-on"))

    # section.rs <-> ui
    edges.append(edge("file:src/section.rs", "file:ui/temperament.slint", "implements"))
    edges.append(edge("file:ui/temperament.slint", "file:src/section.rs", "used-by"))

    # manifest.json -> lib.rs (export_plugin)
    edges.append(edge("file:manifest.json", "file:src/lib.rs", "validates"))

    # test-all.py -> tests/suite.json
    edges.append(edge("file:scripts/test-all.py", "file:tests/suite.json", "reads"))

    # section.rs test -> ui/temperament.slint
    edges.append(edge("file:src/section.rs", "file:ui/temperament.slint", "tests"))

    # contract.rs -> lib.rs (via .oxp)
    edges.append(edge("file:tests/contract.rs", "file:src/lib.rs", "tests"))

    # build-oxp.sh -> manifest.json, ui/
    edges.append(edge("file:build-oxp.sh", "file:manifest.json", "packages"))
    edges.append(edge("file:build-oxp.sh", "file:ui/temperament.slint", "packages"))

    # CI workflow -> scripts
    edges.append(edge("file:.github/workflows/ci.yml", "file:scripts/apt-install.sh", "uses"))
    edges.append(edge("file:.github/workflows/ci.yml", "file:scripts/check-dco.sh", "uses"))
    edges.append(edge("file:.github/workflows/ci.yml", "file:scripts/test-all.py", "uses"))

    # docs.yml -> book.toml
    edges.append(edge("file:.github/workflows/docs.yml", "file:docs/manual/book.toml", "builds"))

    # design.md references source files
    edges.append(edge("file:docs/design.md", "file:src/intonation.rs", "describes"))
    edges.append(edge("file:docs/design.md", "file:src/strings.rs", "describes"))
    edges.append(edge("file:docs/design.md", "file:src/true_temperament.rs", "describes"))
    edges.append(edge("file:docs/design.md", "file:src/perform.rs", "describes"))
    edges.append(edge("file:docs/design.md", "file:src/section.rs", "describes"))
    edges.append(edge("file:docs/design.md", "file:ui/temperament.slint", "describes"))

    # how-it-works.md references source
    edges.append(edge("file:docs/manual/src/how-it-works.md", "file:src/intonation.rs", "describes"))
    edges.append(edge("file:docs/manual/src/how-it-works.md", "file:src/strings.rs", "describes"))
    edges.append(edge("file:docs/manual/src/how-it-works.md", "file:src/perform.rs", "describes"))
    edges.append(edge("file:docs/manual/src/how-it-works.md", "file:src/section.rs", "describes"))

    # features.md references source tests
    edges.append(edge("file:docs/manual/src/features.md", "file:src/intonation.rs", "references-tests"))
    edges.append(edge("file:docs/manual/src/features.md", "file:src/perform.rs", "references-tests"))
    edges.append(edge("file:docs/manual/src/features.md", "file:src/strings.rs", "references-tests"))
    edges.append(edge("file:docs/manual/src/features.md", "file:src/true_temperament.rs", "references-tests"))
    edges.append(edge("file:docs/manual/src/features.md", "file:src/section.rs", "references-tests"))
    edges.append(edge("file:docs/manual/src/features.md", "file:src/settings.rs", "references-tests"))
    edges.append(edge("file:docs/manual/src/features.md", "file:tests/contract.rs", "references-tests"))

    # Cargo.toml -> build-oxp.sh
    edges.append(edge("file:Cargo.toml", "file:build-oxp.sh", "builds"))

    # CI -> Cargo.toml
    edges.append(edge("file:.github/workflows/ci.yml", "file:Cargo.toml", "validates"))

    # deny.toml -> Cargo.toml
    edges.append(edge("file:deny.toml", "file:Cargo.toml", "validates"))

    # dependabot -> Cargo.toml
    edges.append(edge("file:.github/dependabot.yml", "file:Cargo.toml", "monitors"))

    # CLAUDE.md -> test-all.py
    edges.append(edge("file:CLAUDE.md", "file:scripts/test-all.py", "references"))

    # CLAUDE.md -> Cargo.toml
    edges.append(edge("file:CLAUDE.md", "file:Cargo.toml", "references"))

    # CONTRIBUTING.md -> check-dco.sh
    edges.append(edge("file:CONTRIBUTING.md", "file:scripts/check-dco.sh", "references"))

    return files, edges


# ================================================================ main

batches = {
    1: make_batch1,
    2: make_batch2,
    3: make_batch3,
    4: make_batch4,
}

for batch_index, factory in batches.items():
    files, edges = factory()
    output = {
        "batchIndex": batch_index,
        "nodes": files,
        "edges": edges,
    }
    output_path = os.path.join(OUTPUT_DIR, f"batch-{batch_index}.json")
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(output, f, indent=2, ensure_ascii=False)
    print(f"Wrote {output_path} ({len(files)} nodes, {len(edges)} edges)")

print("Done.")
