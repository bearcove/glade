//! Layout page - Separator, AspectRatio, ScrollArea, Carousel, SplitPane

use dioxus::prelude::*;
use glade_dioxus::{
    AspectRatio, AspectRatioPreset, Card, CardContent, Carousel, CarouselNavigation, CarouselSlide,
    Grid, Row, ScrollArea, ScrollDirection, Section, Separator, SeparatorOrientation, Stack,
    SubSection,
};

#[component]
pub fn LayoutPage() -> Element {
    rsx! {
        Section { id: "separator".to_string(), title: "Separator".to_string(),
            SubSection { title: "Horizontal".to_string(),
                Stack {
                    p { "Content above" }
                    Separator {}
                    p { "Content below" }
                }
            }
            SubSection { title: "Vertical".to_string(),
                Row {
                    span { "Left" }
                    Separator { orientation: SeparatorOrientation::Vertical }
                    span { "Right" }
                }
            }
            SubSection { title: "With label".to_string(),
                Separator { label: "OR".to_string() }
            }
        }

        Section { id: "aspect-ratio".to_string(), title: "Aspect Ratio".to_string(),
            SubSection { title: "Presets".to_string(),
                Grid {
                    div {
                        p { "16:9 (Video)" }
                        AspectRatio { ratio: AspectRatioPreset::Video,
                            img {
                                src: "https://picsum.photos/seed/video/640/360",
                                alt: "16:9 placeholder",
                                style: "width: 100%; height: 100%; object-fit: cover;",
                            }
                        }
                    }
                    div {
                        p { "4:3 (Classic)" }
                        AspectRatio { ratio: AspectRatioPreset::Classic,
                            img {
                                src: "https://picsum.photos/seed/classic/400/300",
                                alt: "4:3 placeholder",
                                style: "width: 100%; height: 100%; object-fit: cover;",
                            }
                        }
                    }
                    div {
                        p { "1:1 (Square)" }
                        AspectRatio { ratio: AspectRatioPreset::Square,
                            img {
                                src: "https://picsum.photos/seed/square/300/300",
                                alt: "1:1 placeholder",
                                style: "width: 100%; height: 100%; object-fit: cover;",
                            }
                        }
                    }
                }
            }
        }

        Section { id: "scroll-area".to_string(), title: "Scroll Area".to_string(),
            SubSection { title: "Vertical scroll".to_string(),
                ScrollArea { max_height: "200px".to_string(),
                    Stack {
                        for i in 1..=20 {
                            p { "Line {i} - Lorem ipsum dolor sit amet" }
                        }
                    }
                }
            }
            SubSection { title: "Horizontal scroll".to_string(),
                ScrollArea { direction: ScrollDirection::Horizontal,
                    Row {
                        for i in 1..=10 {
                            Card {
                                CardContent {
                                    div { style: "white-space: nowrap;", "Card {i}" }
                                }
                            }
                        }
                    }
                }
            }
        }

        Section { id: "carousel".to_string(), title: "Carousel".to_string(),
            SubSection { title: "Basic".to_string(),
                Carousel { slides: 3,
                    CarouselSlide {
                        div {
                            style: "background: var(--glade-primary); color: white; padding: 4rem; text-align: center;",
                            "Slide 1"
                        }
                    }
                    CarouselSlide {
                        div {
                            style: "background: var(--glade-success); color: white; padding: 4rem; text-align: center;",
                            "Slide 2"
                        }
                    }
                    CarouselSlide {
                        div {
                            style: "background: var(--glade-warning); color: white; padding: 4rem; text-align: center;",
                            "Slide 3"
                        }
                    }
                }
            }
            SubSection { title: "Navigation variants".to_string(),
                Grid {
                    div {
                        p { "Arrows only" }
                        Carousel { slides: 2, navigation: CarouselNavigation::Arrows,
                            CarouselSlide {
                                div { style: "background: #e0e0e0; padding: 2rem; text-align: center;", "A" }
                            }
                            CarouselSlide {
                                div { style: "background: #d0d0d0; padding: 2rem; text-align: center;", "B" }
                            }
                        }
                    }
                    div {
                        p { "Dots only" }
                        Carousel { slides: 2, navigation: CarouselNavigation::Dots,
                            CarouselSlide {
                                div { style: "background: #e0e0e0; padding: 2rem; text-align: center;", "A" }
                            }
                            CarouselSlide {
                                div { style: "background: #d0d0d0; padding: 2rem; text-align: center;", "B" }
                            }
                        }
                    }
                }
            }
        }

        Section { id: "split-pane".to_string(), title: "Split Pane".to_string(),
            SubSection { title: "Horizontal".to_string(),
                p { "Split pane allows resizable panels - see the component for full implementation." }
            }
        }
    }
}
