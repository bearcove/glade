//! Calendar component for date selection

use dioxus::prelude::*;
use jiff::civil::{Date, Weekday};

stylance::import_style!(style, "calendar.module.scss");

/// Size variants for calendar
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum CalendarSize {
    /// Small calendar
    Small,
    /// Medium calendar (default)
    #[default]
    Medium,
    /// Large calendar
    Large,
}

/// Calendar component for selecting dates
#[component]
pub fn Calendar(
    /// Currently selected date
    #[props(optional)]
    selected: Option<Date>,
    /// Callback when date is selected
    #[props(optional)]
    on_select: Option<EventHandler<Date>>,
    /// Initial month/year to display (defaults to today)
    #[props(optional)]
    initial_date: Option<Date>,
    /// Size variant
    #[props(default)]
    size: CalendarSize,
    /// Minimum selectable date
    #[props(optional)]
    min_date: Option<Date>,
    /// Maximum selectable date
    #[props(optional)]
    max_date: Option<Date>,
    /// Whether the calendar is disabled
    #[props(default = false)]
    disabled: bool,
) -> Element {
    // Default to today if no initial date provided
    let today = Date::try_from(jiff::Zoned::now()).unwrap_or_else(|_| Date::constant(2024, 1, 1));
    let initial = initial_date.or(selected).unwrap_or(today);

    let mut view_year = use_signal(|| initial.year());
    let mut view_month = use_signal(|| initial.month());

    let size_class = match size {
        CalendarSize::Small => style::small,
        CalendarSize::Medium => "",
        CalendarSize::Large => style::large,
    };

    let month_names = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];
    let day_names = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

    let go_prev_month = move |_| {
        let m = view_month();
        let y = view_year();
        if m == 1 {
            view_month.set(12);
            view_year.set(y - 1);
        } else {
            view_month.set(m - 1);
        }
    };

    let go_next_month = move |_| {
        let m = view_month();
        let y = view_year();
        if m == 12 {
            view_month.set(1);
            view_year.set(y + 1);
        } else {
            view_month.set(m + 1);
        }
    };

    let is_date_disabled = |date: Date| -> bool {
        if disabled {
            return true;
        }
        if let Some(min) = min_date {
            if date < min {
                return true;
            }
        }
        if let Some(max) = max_date {
            if date > max {
                return true;
            }
        }
        false
    };

    let year = view_year();
    let month = view_month();

    // Use jiff to get first day of month and days in month
    let first_of_month = Date::new(year, month, 1).unwrap_or_else(|_| Date::constant(2024, 1, 1));
    let num_days = first_of_month.days_in_month();

    // Get weekday of first day (Sunday = 0)
    let first_weekday = match first_of_month.weekday() {
        Weekday::Sunday => 0,
        Weekday::Monday => 1,
        Weekday::Tuesday => 2,
        Weekday::Wednesday => 3,
        Weekday::Thursday => 4,
        Weekday::Friday => 5,
        Weekday::Saturday => 6,
    };

    // Build calendar grid
    let mut days: Vec<Option<i8>> = Vec::new();
    // Add empty cells for days before first of month
    for _ in 0..first_weekday {
        days.push(None);
    }
    // Add days of month
    for d in 1..=num_days {
        days.push(Some(d));
    }
    // Pad to complete last week
    while days.len() % 7 != 0 {
        days.push(None);
    }

    rsx! {
        div {
            class: stylance::classes!(style::calendar, size_class, if disabled { style::disabled } else { "" }),

            // Header with month/year and navigation
            div { class: style::header,
                button {
                    class: style::nav_button,
                    onclick: go_prev_month,
                    disabled,
                    aria_label: "Previous month",
                    svg {
                        width: "16",
                        height: "16",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }

                span { class: style::month_year,
                    "{month_names[(month - 1) as usize]} {year}"
                }

                button {
                    class: style::nav_button,
                    onclick: go_next_month,
                    disabled,
                    aria_label: "Next month",
                    svg {
                        width: "16",
                        height: "16",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "9 18 15 12 9 6" }
                    }
                }
            }

            // Day names header
            div { class: style::weekdays,
                for day_name in day_names {
                    span { class: style::weekday, "{day_name}" }
                }
            }

            // Calendar grid
            div { class: style::grid,
                for (idx, day_opt) in days.iter().enumerate() {
                    match day_opt {
                        Some(day) => {
                            let date = Date::new(year, month, *day).unwrap_or(first_of_month);
                            let is_selected = selected.map(|s| s == date).unwrap_or(false);
                            let is_disabled = is_date_disabled(date);
                            let is_today = date == today;

                            rsx! {
                                button {
                                    key: "day-{idx}",
                                    class: stylance::classes!(
                                        style::day,
                                        if is_selected { style::selected } else { "" },
                                        if is_today && !is_selected { style::today } else { "" }
                                    ),
                                    disabled: is_disabled,
                                    onclick: move |_| {
                                        if let Some(handler) = &on_select {
                                            handler.call(date);
                                        }
                                    },
                                    "{day}"
                                }
                            }
                        }
                        None => {
                            rsx! {
                                span { key: "empty-{idx}", class: style::day_empty }
                            }
                        }
                    }
                }
            }
        }
    }
}
