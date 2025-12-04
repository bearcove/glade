//! Primitives page - Icons, Button, Badge, Card, Avatar

use dioxus::prelude::*;
use glade_dioxus::{
    Avatar, AvatarGroup, AvatarSize, Badge, BadgeSize, BadgeVariant, Button, ButtonGroup,
    ButtonSize, ButtonVariant, Card, CardContent, CardDescription, CardFooter, CardHeader,
    CardTitle, IconAlertCircle, IconCheck, IconChevronDown, IconChevronRight, IconCircleCheck,
    IconCircleX, IconExternalLink, IconGithub, IconGrid, IconGridItem, IconInfo, IconLoader,
    IconMenu, IconMinus, IconPlus, IconSearch, IconTriangleAlert, IconX, Row, RowAlign, Section,
    SubSection,
};

#[component]
pub fn PrimitivesPage() -> Element {
    rsx! {
        Section { id: "icons".to_string(), title: "Icons".to_string(),
            p {
                "Icons are provided by "
                a { href: "https://lucide.dev", target: "_blank", "Lucide" }
                ". SVGs are embedded at compile time."
            }
            SubSection { title: "Actions".to_string(),
                IconGrid {
                    IconGridItem { name: "IconSearch".to_string(), IconSearch {} }
                    IconGridItem { name: "IconPlus".to_string(), IconPlus {} }
                    IconGridItem { name: "IconMinus".to_string(), IconMinus {} }
                    IconGridItem { name: "IconCheck".to_string(), IconCheck {} }
                    IconGridItem { name: "IconX".to_string(), IconX {} }
                }
            }
            SubSection { title: "Navigation".to_string(),
                IconGrid {
                    IconGridItem { name: "IconMenu".to_string(), IconMenu {} }
                    IconGridItem { name: "IconChevronDown".to_string(), IconChevronDown {} }
                    IconGridItem { name: "IconChevronRight".to_string(), IconChevronRight {} }
                    IconGridItem { name: "IconExternalLink".to_string(), IconExternalLink {} }
                }
            }
            SubSection { title: "Status".to_string(),
                IconGrid {
                    IconGridItem { name: "IconInfo".to_string(), IconInfo {} }
                    IconGridItem { name: "IconAlertCircle".to_string(), IconAlertCircle {} }
                    IconGridItem { name: "IconCircleCheck".to_string(), IconCircleCheck {} }
                    IconGridItem { name: "IconCircleX".to_string(), IconCircleX {} }
                    IconGridItem { name: "IconTriangleAlert".to_string(), IconTriangleAlert {} }
                    IconGridItem { name: "IconLoader".to_string(), IconLoader {} }
                }
            }
            SubSection { title: "Objects".to_string(),
                IconGrid {
                    IconGridItem { name: "IconGithub".to_string(), IconGithub {} }
                }
            }
        }

        Section { id: "button".to_string(), title: "Button".to_string(),
            SubSection { title: "Variants".to_string(),
                Row {
                    Button { variant: ButtonVariant::Primary, "Primary" }
                    Button { variant: ButtonVariant::Secondary, "Secondary" }
                    Button { variant: ButtonVariant::Danger, "Danger" }
                    Button { variant: ButtonVariant::Ghost, "Ghost" }
                }
            }
            SubSection { title: "Sizes".to_string(),
                Row { align: RowAlign::Center,
                    Button { size: ButtonSize::Small, "Small" }
                    Button { size: ButtonSize::Medium, "Medium" }
                    Button { size: ButtonSize::Large, "Large" }
                }
            }
            SubSection { title: "Disabled".to_string(),
                Row {
                    Button { variant: ButtonVariant::Primary, disabled: true, "Disabled" }
                }
            }
            SubSection { title: "Loading".to_string(),
                Row {
                    Button { variant: ButtonVariant::Primary, loading: true, "Save Changes" }
                    Button { variant: ButtonVariant::Secondary, loading: true, "Loading" }
                    Button { variant: ButtonVariant::Danger, loading: true, "Delete" }
                }
            }
            SubSection { title: "With Icons".to_string(),
                Row {
                    Button { variant: ButtonVariant::Primary, IconPlus {} " New Item" }
                    Button { variant: ButtonVariant::Secondary, IconSearch {} " Search" }
                    Button { variant: ButtonVariant::Danger, IconX {} " Remove" }
                    Button { variant: ButtonVariant::Ghost, IconExternalLink {} " Open Link" }
                }
            }
        }

        Section { id: "button-group".to_string(), title: "Button Group".to_string(),
            SubSection { title: "Basic".to_string(),
                Row {
                    ButtonGroup {
                        Button { variant: ButtonVariant::Secondary, "Left" }
                        Button { variant: ButtonVariant::Secondary, "Middle" }
                        Button { variant: ButtonVariant::Secondary, "Right" }
                    }
                }
            }
        }

        Section { id: "avatar".to_string(), title: "Avatar".to_string(),
            SubSection { title: "Sizes".to_string(),
                Row { align: RowAlign::Center,
                    Avatar { size: AvatarSize::Small, initials: "SM".to_string() }
                    Avatar { size: AvatarSize::Medium, initials: "MD".to_string() }
                    Avatar { size: AvatarSize::Large, initials: "LG".to_string() }
                }
            }
            SubSection { title: "With Image".to_string(),
                Row { align: RowAlign::Center,
                    Avatar { src: "https://i.pravatar.cc/150?img=1".to_string(), size: AvatarSize::Medium }
                    Avatar { src: "https://i.pravatar.cc/150?img=2".to_string(), size: AvatarSize::Medium }
                    Avatar { src: "https://i.pravatar.cc/150?img=3".to_string(), size: AvatarSize::Medium }
                }
            }
            SubSection { title: "Avatar Group".to_string(),
                AvatarGroup {
                    Avatar { src: "https://i.pravatar.cc/150?img=1".to_string(), size: AvatarSize::Medium }
                    Avatar { src: "https://i.pravatar.cc/150?img=2".to_string(), size: AvatarSize::Medium }
                    Avatar { src: "https://i.pravatar.cc/150?img=3".to_string(), size: AvatarSize::Medium }
                    Avatar { initials: "+3".to_string(), size: AvatarSize::Medium }
                }
            }
        }

        Section { id: "badge".to_string(), title: "Badge".to_string(),
            SubSection { title: "Variants".to_string(),
                Row {
                    Badge { variant: BadgeVariant::Default, "Default" }
                    Badge { variant: BadgeVariant::Primary, "Primary" }
                    Badge { variant: BadgeVariant::Success, "Success" }
                    Badge { variant: BadgeVariant::Warning, "Warning" }
                    Badge { variant: BadgeVariant::Error, "Error" }
                }
            }
            SubSection { title: "Sizes".to_string(),
                Row { align: RowAlign::Center,
                    Badge { size: BadgeSize::Small, "Small" }
                    Badge { size: BadgeSize::Medium, "Medium" }
                    Badge { size: BadgeSize::Large, "Large" }
                }
            }
        }

        Section { id: "card".to_string(), title: "Card".to_string(),
            SubSection { title: "Basic".to_string(),
                Card {
                    CardHeader {
                        CardTitle { "Card Title" }
                        CardDescription { "Card description goes here" }
                    }
                    CardContent {
                        p { "This is the main content of the card. It can contain any elements." }
                    }
                    CardFooter {
                        Button { variant: ButtonVariant::Primary, "Action" }
                    }
                }
            }
        }
    }
}
