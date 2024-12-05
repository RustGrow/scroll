use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a {
                    target: "_blank",
                    href: "https://dioxuslabs.com/learn/0.6/",
                    "📚 Learn Dioxus"
                }
                a { target: "_blank", href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a {
                    target: "_blank",
                    href: "https://github.com/dioxus-community/",
                    "📡 Community Libraries"
                }
                a {
                    target: "_blank",
                    href: "https://github.com/DioxusLabs/sdk",
                    "⚙️ Dioxus Development Kit"
                }
                a {
                    target: "_blank",
                    href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a { target: "_blank", href: "https://discord.gg/XgGxMSkvUM",
                    "👋 Community Discord"
                }
                div { class: " bg-green-400", "next" }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div { id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p {
                "In blog #{id}, we show how the Dioxus router works and how URL paramaters can be passed as props to our route components."
            }

            // Navigation links
            Link { to: Route::Blog { id: id - 1 }, "Previous" }
            span { " <---> " }
            Link { to: Route::Blog { id: id + 1 }, "Next" }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}
