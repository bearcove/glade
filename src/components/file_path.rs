//! File path inline component for displaying file references

use dioxus::prelude::*;

use super::file_icons;

stylance::import_style!(style, "file_path.module.scss");

/// Get the file extension from a path
fn get_extension(path: &str) -> Option<&str> {
    path.rsplit('.').next().filter(|ext| !ext.contains('/'))
}

/// Map file extension to icon type
fn ext_to_icon_type(ext: &str) -> &'static str {
    match ext.to_lowercase().as_str() {
        "rs" => "rust",
        "ts" | "mts" | "cts" | "tsx" => "typescript",
        "js" | "mjs" | "cjs" | "jsx" => "javascript",
        "py" | "pyw" | "pyi" => "python",
        "c" | "h" => "c",
        "cpp" | "hpp" | "cc" | "hh" | "cxx" | "hxx" => "cpp",
        "go" => "go",
        "java" => "java",
        "kt" | "kts" => "kotlin",
        "rb" | "erb" => "ruby",
        "php" => "php",
        "swift" => "swift",
        "cs" => "csharp",
        "lua" => "lua",
        "pl" | "pm" => "perl",
        "r" => "r",
        "scala" | "sc" => "scala",
        "ex" | "exs" => "elixir",
        "hs" | "lhs" => "haskell",
        "clj" | "cljs" | "cljc" | "edn" => "clojure",
        "zig" => "zig",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "xml" | "svg" => "xml",
        "sql" => "sql",
        "html" | "htm" => "html",
        "css" => "css",
        "scss" | "sass" => "scss",
        "md" | "mdx" | "markdown" => "markdown",
        "sh" | "bash" | "zsh" | "fish" => "bash",
        "gitignore" | "gitattributes" | "gitmodules" => "git",
        "j2" | "jinja" | "jinja2" => "jinja",
        _ => "file",
    }
}

/// Returns the appropriate icon SVG based on file extension
fn get_file_icon_svg(ext: &str) -> &'static str {
    let icon_type = ext_to_icon_type(ext);
    file_icons::get_icon_svg(icon_type).unwrap_or_else(|| file_icons::get_icon_svg("file").unwrap())
}

/// Inline file path display with optional line number and file type icon
#[component]
pub fn FilePath(
    /// The file path
    path: String,
    /// Optional line number
    #[props(optional)]
    line: Option<u32>,
    /// Optional column number (only shown if line is also present)
    #[props(optional)]
    column: Option<u32>,
    /// Click handler (e.g., to open the file) - receives the full location string
    #[props(optional)]
    on_click: Option<EventHandler<String>>,
) -> Element {
    let location_string = {
        let mut s = path.clone();
        if let Some(l) = line {
            s.push_str(&format!(":{}", l));
            if let Some(c) = column {
                s.push_str(&format!(":{}", c));
            }
        }
        s
    };
    let location_for_click = location_string.clone();

    let handle_click = move |_| {
        if let Some(callback) = &on_click {
            callback.call(location_for_click.clone());
        }
    };

    let has_click = on_click.is_some();
    let class = if has_click {
        stylance::classes!(style::file_path, style::clickable)
    } else {
        style::file_path.to_string()
    };

    let ext = get_extension(&path).unwrap_or("").to_string();
    let icon_svg = get_file_icon_svg(&ext);

    rsx! {
        span { class: class, onclick: handle_click,
            span { class: style::icon, dangerous_inner_html: icon_svg }
            span { class: style::path, {path} }
            {line.map(|l| rsx! {
                span { class: style::location,
                    ":" span { class: style::line, "{l}" }
                    {column.map(|c| rsx! {
                        ":" span { class: style::column, "{c}" }
                    })}
                }
            })}
        }
    }
}
