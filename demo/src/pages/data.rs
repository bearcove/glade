//! Data page - DiffStats, CodeExecutionResult, ToolCallBadge, CodeBlock, Table, Stat, List, Descriptions, HoverCard, Popover

use dioxus::prelude::*;
use glade_dioxus::{
    Button, CodeBlock, CodeExecutionResult, DescriptionItem, Descriptions, DescriptionsLayout,
    DiffFileSummary, DiffStats, DiffStatsStyle, ExecutionStatus, Grid, HoverCard,
    HoverCardPosition, Language, List, ListItem, Popover, PopoverContent, PopoverPosition, Row,
    RowAlign, Section, Stack, Stat, StatCard, StatGroup, StatTrend, SubSection, Table, TableBody,
    TableCell, TableHeader, TableHeaderCell, TableRow, ToolCallBadge, ToolCallStatus,
};

#[component]
pub fn DataPage() -> Element {
    rsx! {
        Section { id: "diff-stats".to_string(), title: "Diff Stats".to_string(),
            SubSection { title: "Inline".to_string(),
                Row { align: RowAlign::Center,
                    DiffStats { additions: 10, deletions: 3 }
                    DiffStats { additions: 50, deletions: 20 }
                    DiffStats { additions: 0, deletions: 15 }
                    DiffStats { additions: 25, deletions: 0 }
                    DiffStats { additions: 0, deletions: 0 }
                }
            }
            SubSection { title: "Full (with bar)".to_string(),
                Stack {
                    DiffStats { additions: 10, deletions: 3, style_variant: DiffStatsStyle::Full }
                    DiffStats { additions: 50, deletions: 20, style_variant: DiffStatsStyle::Full }
                    DiffStats { additions: 5, deletions: 45, style_variant: DiffStatsStyle::Full }
                }
            }
            SubSection { title: "File Summary".to_string(),
                Stack {
                    DiffFileSummary {
                        filename: "src/components/button.rs".to_string(),
                        additions: 25,
                        deletions: 10,
                        status: Some("modified".to_string()),
                    }
                    DiffFileSummary {
                        filename: "src/components/new_widget.rs".to_string(),
                        additions: 150,
                        deletions: 0,
                        status: Some("added".to_string()),
                    }
                    DiffFileSummary {
                        filename: "src/old_module.rs".to_string(),
                        additions: 0,
                        deletions: 80,
                        status: Some("deleted".to_string()),
                    }
                    DiffFileSummary {
                        filename: "src/utils.rs".to_string(),
                        additions: 5,
                        deletions: 5,
                        status: Some("renamed".to_string()),
                    }
                }
            }
        }

        Section { id: "tool-call-badge".to_string(), title: "Tool Call Badge".to_string(),
            SubSection { title: "Statuses".to_string(),
                Row { align: RowAlign::Center,
                    ToolCallBadge { name: "read_file".to_string(), status: ToolCallStatus::Pending }
                    ToolCallBadge { name: "search_code".to_string(), status: ToolCallStatus::Running }
                    ToolCallBadge { name: "write_file".to_string(), status: ToolCallStatus::Success }
                    ToolCallBadge { name: "execute".to_string(), status: ToolCallStatus::Error }
                    ToolCallBadge { name: "deploy".to_string(), status: ToolCallStatus::Cancelled }
                }
            }
            SubSection { title: "Compact".to_string(),
                Row { align: RowAlign::Center,
                    ToolCallBadge { name: "read".to_string(), status: ToolCallStatus::Pending, compact: true }
                    ToolCallBadge { name: "search".to_string(), status: ToolCallStatus::Running, compact: true }
                    ToolCallBadge { name: "write".to_string(), status: ToolCallStatus::Success, compact: true }
                    ToolCallBadge { name: "exec".to_string(), status: ToolCallStatus::Error, compact: true }
                }
            }
        }

        Section { id: "code-execution-result".to_string(), title: "Code Execution Result".to_string(),
            SubSection { title: "Running".to_string(),
                CodeExecutionResult {
                    status: ExecutionStatus::Running,
                    stdout: "Building project...\nCompiling dependencies...".to_string(),
                }
            }
            SubSection { title: "Success".to_string(),
                CodeExecutionResult {
                    status: ExecutionStatus::Success,
                    exit_code: Some(0),
                    duration: Some("1.2s".to_string()),
                    stdout: "All tests passed!\n\n  Running 15 tests\n  15 passed, 0 failed".to_string(),
                }
            }
            SubSection { title: "Failure".to_string(),
                CodeExecutionResult {
                    status: ExecutionStatus::Failure,
                    exit_code: Some(1),
                    duration: Some("0.8s".to_string()),
                    stdout: "Running tests...".to_string(),
                    stderr: "error[E0308]: mismatched types\n  --> src/main.rs:10:5\n   |\n10 |     foo()\n   |     ^^^^^ expected `i32`, found `()`".to_string(),
                }
            }
        }

        Section { id: "code-block".to_string(), title: "Code Block".to_string(),
            SubSection { title: "Rust".to_string(),
                CodeBlock {
                    language: Some(Language::Rust),
                    code: r#"fn main() {
    let message = "Hello, world!";
    println!("{}", message);
}"#.to_string(),
                }
            }
            SubSection { title: "With Filename".to_string(),
                CodeBlock {
                    language: Some(Language::TypeScript),
                    filename: Some("src/app.ts".to_string()),
                    code: r#"export function greet(name: string): string {
    return `Hello, ${name}!`;
}"#.to_string(),
                }
            }
            SubSection { title: "With Line Numbers".to_string(),
                CodeBlock {
                    language: Some(Language::Python),
                    show_line_numbers: true,
                    code: r#"def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

print(fibonacci(10))"#.to_string(),
                }
            }
        }

        Section { id: "stat".to_string(), title: "Stat".to_string(),
            SubSection { title: "Basic stats".to_string(),
                StatGroup {
                    Stat { label: "Total Users".to_string(), value: "12,345".to_string() }
                    Stat { label: "Revenue".to_string(), value: "$45,678".to_string(), trend: StatTrend::Up, change: "+12.5%".to_string() }
                    Stat { label: "Bounce Rate".to_string(), value: "23.4%".to_string(), trend: StatTrend::Down, change: "-5.2%".to_string() }
                }
            }
            SubSection { title: "Stat cards".to_string(),
                Grid {
                    StatCard { label: "Active Users".to_string(), value: "1,234".to_string(), trend: StatTrend::Up, change: "+8%".to_string() }
                    StatCard { label: "Sessions".to_string(), value: "5,678".to_string(), trend: StatTrend::Neutral, change: "0%".to_string() }
                    StatCard { label: "Errors".to_string(), value: "23".to_string(), trend: StatTrend::Down, change: "-15%".to_string() }
                }
            }
        }

        Section { id: "list".to_string(), title: "List".to_string(),
            SubSection { title: "Basic list".to_string(),
                List {
                    ListItem { "First item" }
                    ListItem { "Second item" }
                    ListItem { "Third item" }
                }
            }
            SubSection { title: "Clickable list".to_string(),
                List {
                    ListItem { clickable: true, "Settings" }
                    ListItem { clickable: true, "Profile" }
                    ListItem { clickable: true, selected: true, "Security" }
                }
            }
        }

        Section { id: "descriptions".to_string(), title: "Descriptions".to_string(),
            SubSection { title: "Horizontal layout".to_string(),
                Descriptions { title: "User Details".to_string(),
                    DescriptionItem { label: "Name".to_string(), "John Doe" }
                    DescriptionItem { label: "Email".to_string(), "john@example.com" }
                    DescriptionItem { label: "Phone".to_string(), "+1 (555) 123-4567" }
                    DescriptionItem { label: "Address".to_string(), "123 Main St, City" }
                }
            }
            SubSection { title: "Vertical layout".to_string(),
                Descriptions { layout: DescriptionsLayout::Vertical,
                    DescriptionItem { label: "Status".to_string(), "Active" }
                    DescriptionItem { label: "Created".to_string(), "Jan 15, 2024" }
                    DescriptionItem { label: "Updated".to_string(), "Mar 20, 2024" }
                }
            }
        }

        Section { id: "table".to_string(), title: "Table".to_string(),
            Table {
                TableHeader {
                    TableHeaderCell { "Name" }
                    TableHeaderCell { "Email" }
                    TableHeaderCell { "Role" }
                }
                TableBody {
                    TableRow {
                        TableCell { "Alice Johnson" }
                        TableCell { "alice@example.com" }
                        TableCell { "Admin" }
                    }
                    TableRow {
                        TableCell { "Bob Smith" }
                        TableCell { "bob@example.com" }
                        TableCell { "User" }
                    }
                    TableRow {
                        TableCell { "Carol White" }
                        TableCell { "carol@example.com" }
                        TableCell { "Editor" }
                    }
                }
            }
        }

        Section { id: "hover-card".to_string(), title: "Hover Card".to_string(),
            SubSection { title: "User profile".to_string(),
                HoverCard { position: HoverCardPosition::Bottom,
                    trigger: rsx! {
                        a { href: "#", style: "color: var(--glade-primary);", "@johndoe" }
                    },
                    div { style: "display: flex; align-items: center; gap: 0.75rem;",
                        img {
                            src: "https://i.pravatar.cc/96?u=johndoe",
                            alt: "John Doe",
                            style: "width: 3rem; height: 3rem; border-radius: 50%; object-fit: cover;"
                        }
                        div {
                            strong { "John Doe" }
                            div { style: "font-size: 0.875rem; color: var(--glade-text-muted);", "@johndoe" }
                        }
                    }
                    p { style: "margin: 0.5rem 0 0; font-size: 0.875rem;", "Software engineer building great things." }
                }
            }
        }

        Section { id: "popover".to_string(), title: "Popover".to_string(),
            SubSection { title: "Click to open".to_string(),
                Popover { position: PopoverPosition::Bottom,
                    trigger: rsx! { Button { "Open Popover" } },
                    PopoverContent {
                        Stack {
                            strong { "Popover Title" }
                            p { "This is popover content that appears on click." }
                        }
                    }
                }
            }
        }
    }
}
