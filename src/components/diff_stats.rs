//! Git diff statistics display component

use dioxus::prelude::*;

stylance::import_style!(style, "diff_stats.module.scss");

/// Display style for diff stats
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffStatsStyle {
    /// Compact inline display
    #[default]
    Inline,
    /// Full display with bar graph
    Full,
}

/// Git diff statistics display
#[component]
pub fn DiffStats(
    /// Number of lines added
    #[props(default = 0)]
    additions: usize,
    /// Number of lines deleted
    #[props(default = 0)]
    deletions: usize,
    /// Display style
    #[props(default)]
    style_variant: DiffStatsStyle,
    /// Maximum bar width (number of blocks)
    #[props(default = 5)]
    max_blocks: usize,
) -> Element {
    let total = additions + deletions;

    match style_variant {
        DiffStatsStyle::Inline => rsx! {
            span { class: style::inline,
                if additions > 0 {
                    span { class: style::additions, "+{additions}" }
                }
                if deletions > 0 {
                    span { class: style::deletions, "-{deletions}" }
                }
                if total == 0 {
                    span { class: style::no_changes, "0" }
                }
            }
        },
        DiffStatsStyle::Full => {
            let add_blocks = if total > 0 {
                (additions * max_blocks / total).max(usize::from(additions > 0))
            } else {
                0
            };
            let del_blocks = if total > 0 {
                (deletions * max_blocks / total).max(usize::from(deletions > 0))
            } else {
                0
            };
            let empty_blocks = max_blocks.saturating_sub(add_blocks + del_blocks);

            rsx! {
                span { class: style::full,
                    span { class: style::additions, "+{additions}" }
                    span { class: style::deletions, "-{deletions}" }
                    span { class: style::bar,
                        for _ in 0..add_blocks {
                            span { class: style::block_add }
                        }
                        for _ in 0..del_blocks {
                            span { class: style::block_del }
                        }
                        for _ in 0..empty_blocks {
                            span { class: style::block_empty }
                        }
                    }
                }
            }
        }
    }
}

/// A single file's diff summary
#[component]
pub fn DiffFileSummary(
    /// File path
    filename: String,
    /// Number of additions
    #[props(default = 0)]
    additions: usize,
    /// Number of deletions
    #[props(default = 0)]
    deletions: usize,
    /// File status (added, modified, deleted, renamed)
    status: Option<String>,
) -> Element {
    let status_class = match status.as_deref() {
        Some("added") => style::status_added,
        Some("deleted") => style::status_deleted,
        Some("renamed") => style::status_renamed,
        _ => style::status_modified,
    };

    let status_letter = match status.as_deref() {
        Some("added") => "A",
        Some("deleted") => "D",
        Some("renamed") => "R",
        _ => "M",
    };

    rsx! {
        div { class: style::file_summary,
            span { class: stylance::classes!(style::file_status, status_class), "{status_letter}" }
            span { class: style::filename, "{filename}" }
            DiffStats { additions, deletions, style_variant: DiffStatsStyle::Full }
        }
    }
}
