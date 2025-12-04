//! Form page - Input, Textarea, Select, Checkbox, Radio, Toggle, Slider, FileInput, Rating, Label, Calendar

use dioxus::prelude::*;
use glade_dioxus::{
    Calendar, CalendarSize, Checkbox, FileInput, FormField, Grid, Input, Label, Radio, RadioGroup,
    Rating, RatingSize, Row, RowAlign, Section, SegmentedInput, Select, Slider, SliderSize, Stack,
    SubSection, Toggle, ToggleSize, Textarea,
};

#[component]
pub fn FormPage() -> Element {
    let mut checkbox_checked = use_signal(|| false);
    let mut radio_value = use_signal(|| "option1".to_string());
    let mut switch_checked = use_signal(|| false);
    let mut slider_value = use_signal(|| 50.0_f64);
    let mut rating_value = use_signal(|| 3);
    let mut otp_value = use_signal(|| String::new());

    rsx! {
        Section { id: "input".to_string(), title: "Input".to_string(),
            SubSection { title: "Basic".to_string(),
                Grid {
                    Input { label: "Email".to_string(), placeholder: "you@example.com".to_string() }
                    Input { label: "Password".to_string(), r#type: "password".to_string(), placeholder: "Enter password".to_string() }
                    Input { label: "With error".to_string(), error: "This field is required".to_string(), required: true }
                    Input { label: "Disabled".to_string(), disabled: true, value: "Cannot edit".to_string() }
                }
            }
        }

        Section { id: "textarea".to_string(), title: "Textarea".to_string(),
            Grid {
                Textarea { label: "Message".to_string(), placeholder: "Type your message...".to_string() }
                Textarea { label: "With error".to_string(), error: "Too short".to_string(), rows: 3 }
            }
        }

        Section { id: "select".to_string(), title: "Select".to_string(),
            Grid {
                Select { label: "Country".to_string(),
                    option { value: "", "Select a country" }
                    option { value: "us", "United States" }
                    option { value: "ca", "Canada" }
                    option { value: "uk", "United Kingdom" }
                }
                Select { label: "Disabled".to_string(), disabled: true,
                    option { "Cannot change" }
                }
            }
        }

        Section { id: "checkbox".to_string(), title: "Checkbox".to_string(),
            Row {
                Checkbox {
                    label: "Accept terms".to_string(),
                    checked: checkbox_checked(),
                    onchange: move |_| checkbox_checked.set(!checkbox_checked()),
                }
                Checkbox { label: "Disabled".to_string(), disabled: true }
                Checkbox { label: "Checked disabled".to_string(), checked: true, disabled: true }
            }
        }

        Section { id: "radio".to_string(), title: "Radio".to_string(),
            RadioGroup { label: "Select an option".to_string(),
                Radio {
                    name: "demo-radio".to_string(),
                    value: "option1".to_string(),
                    label: "Option 1".to_string(),
                    checked: radio_value() == "option1",
                    onchange: move |_| radio_value.set("option1".to_string()),
                }
                Radio {
                    name: "demo-radio".to_string(),
                    value: "option2".to_string(),
                    label: "Option 2".to_string(),
                    checked: radio_value() == "option2",
                    onchange: move |_| radio_value.set("option2".to_string()),
                }
                Radio {
                    name: "demo-radio".to_string(),
                    value: "option3".to_string(),
                    label: "Option 3".to_string(),
                    checked: radio_value() == "option3",
                    onchange: move |_| radio_value.set("option3".to_string()),
                }
            }
        }

        Section { id: "toggle".to_string(), title: "Toggle".to_string(),
            SubSection { title: "Basic".to_string(),
                Row {
                    Toggle {
                        checked: switch_checked(),
                        onchange: move |val| switch_checked.set(val),
                    }
                    Toggle {
                        label: "With label".to_string(),
                        checked: switch_checked(),
                        onchange: move |val| switch_checked.set(val),
                    }
                }
            }
            SubSection { title: "Sizes".to_string(),
                Row { align: RowAlign::Center,
                    Toggle { size: ToggleSize::Small, label: "Small".to_string() }
                    Toggle { size: ToggleSize::Medium, label: "Medium".to_string() }
                    Toggle { size: ToggleSize::Large, label: "Large".to_string() }
                }
            }
            SubSection { title: "Disabled".to_string(),
                Row {
                    Toggle { disabled: true, label: "Disabled off".to_string() }
                    Toggle { disabled: true, checked: true, label: "Disabled on".to_string() }
                }
            }
        }

        Section { id: "slider".to_string(), title: "Slider".to_string(),
            SubSection { title: "Basic".to_string(),
                Stack {
                    Slider {
                        value: slider_value,
                        on_change: move |v| slider_value.set(v),
                        label: "Value".to_string(),
                        show_value: true,
                    }
                    p { "Current value: " {format!("{:.0}", slider_value())} }
                }
            }
            SubSection { title: "Sizes".to_string(),
                Stack {
                    Slider {
                        value: slider_value,
                        on_change: move |v| slider_value.set(v),
                        size: SliderSize::Small,
                        label: "Small".to_string(),
                    }
                    Slider {
                        value: slider_value,
                        on_change: move |v| slider_value.set(v),
                        size: SliderSize::Medium,
                        label: "Medium".to_string(),
                    }
                    Slider {
                        value: slider_value,
                        on_change: move |v| slider_value.set(v),
                        size: SliderSize::Large,
                        label: "Large".to_string(),
                    }
                }
            }
            SubSection { title: "Disabled".to_string(),
                Slider {
                    value: slider_value,
                    label: "Disabled slider".to_string(),
                    disabled: true,
                    show_value: true,
                }
            }
        }

        Section { id: "file-input".to_string(), title: "File Input".to_string(),
            SubSection { title: "Basic".to_string(),
                Stack {
                    FileInput {
                        accept: "*/*".to_string(),
                        on_change: move |_| {},
                    }
                    FileInput {
                        accept: "image/*".to_string(),
                        placeholder: "PNG, JPG, GIF up to 10MB".to_string(),
                        on_change: move |_| {},
                    }
                }
            }
            SubSection { title: "Multiple files".to_string(),
                FileInput {
                    multiple: true,
                    on_change: move |_| {},
                }
            }
        }

        Section { id: "rating".to_string(), title: "Rating".to_string(),
            SubSection { title: "Interactive".to_string(),
                Stack {
                    Rating {
                        value: rating_value() as f64,
                        on_change: move |v: f64| rating_value.set(v as i32),
                    }
                    p { "Rating: " {rating_value().to_string()} " stars" }
                }
            }
            SubSection { title: "Sizes".to_string(),
                Stack {
                    Rating { value: 3.0, size: RatingSize::Small }
                    Rating { value: 3.0, size: RatingSize::Medium }
                    Rating { value: 3.0, size: RatingSize::Large }
                }
            }
            SubSection { title: "Read-only".to_string(),
                Rating { value: 4.0, readonly: true }
            }
        }

        Section { id: "label".to_string(), title: "Label".to_string(),
            SubSection { title: "Basic".to_string(),
                Stack {
                    Label { "Username" }
                    Label { required: true, "Email" }
                    Label { optional: true, "Nickname" }
                }
            }
            SubSection { title: "With FormField".to_string(),
                FormField { label: "Email address".to_string(), id: "email-field".to_string(), required: true,
                    Input { placeholder: "you@example.com".to_string() }
                }
            }
        }

        Section { id: "segmented-input".to_string(), title: "Segmented Input".to_string(),
            SubSection { title: "OTP / Verification code".to_string(),
                Stack {
                    SegmentedInput {
                        length: 6,
                        value: otp_value(),
                        on_change: move |v| otp_value.set(v),
                    }
                    p { "Value: " {otp_value().to_string()} }
                }
            }
            SubSection { title: "4-digit PIN".to_string(),
                SegmentedInput { length: 4 }
            }
        }

        Section { id: "calendar".to_string(), title: "Calendar".to_string(),
            SubSection { title: "Basic".to_string(),
                Calendar {}
            }
            SubSection { title: "Sizes".to_string(),
                Row {
                    Calendar { size: CalendarSize::Small }
                    Calendar { size: CalendarSize::Medium }
                }
            }
        }
    }
}
