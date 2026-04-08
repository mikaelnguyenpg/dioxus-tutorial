use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

        div { class: "test-section",
            h2 { "1. SVG Test" }
            SvgExample {}
        }

        div { class: "test-section",
            h2 { "2. Canvas Test (HTML5 Canvas)" }
            CanvasExample {}
        }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

// ==================== SVG TEST ====================
#[component]
fn SvgExample() -> Element {
    rsx! {
        div { class: "svg-test",
            // Cách 1: Dùng <svg> trực tiếp trong rsx
            svg {
                width: "200",
                height: "200",
                view_box: "0 0 200 200",
                circle {
                    cx: "100",
                    cy: "100",
                    r: "80",
                    fill: "none",
                    stroke: "#3b82f6",
                    stroke_width: "15"
                }
                text {
                    x: "100",
                    y: "105",
                    text_anchor: "middle",
                    font_size: "24",
                    fill: "#1e40af",
                    "Dioxus"
                }
            }

            // Cách 2: Dùng img với file SVG (đã có sẵn)
            img {
                src: HEADER_SVG,
                width: "180",
                alt: "Header SVG"
            }
        }
    }
}

// ==================== CANVAS TEST ====================
#[component]
fn CanvasExample() -> Element {
    let mut count = use_signal(|| 0);

    // Dùng dioxus::document::eval (cách đúng trong Dioxus 0.7)
    use_effect(move || {
        let script = format!(
            r#"
            const canvas = document.getElementById('canvas-test');
            if (!canvas) return;
            const ctx = canvas.getContext('2d');

            ctx.clearRect(0, 0, canvas.width, canvas.height);

            // Background
            ctx.fillStyle = '#3b82f6';
            ctx.fillRect(20, 20, 260, 160);

            // Text
            ctx.fillStyle = '#ffffff';
            ctx.font = 'bold 28px Arial';
            ctx.fillText('Hello Canvas!', 50, 80);

            ctx.fillStyle = '#111111';
            ctx.font = '18px Arial';
            ctx.fillText('Dioxus Desktop', 55, 120);

            ctx.fillStyle = '#ffffff';
            ctx.font = 'bold 20px Arial';
            ctx.fillText('Count: {}', 105, 165);
            "#,
            count()
        );

        // Cách gọi eval đúng trong Dioxus 0.7
        dioxus::document::eval(&script);
    });

    rsx! {
        div {
            canvas {
                id: "canvas-test",
                width: "300",
                height: "200",
                style: "border: 2px solid #3b82f6; border-radius: 8px; background: white;"
            }
            br {}
            button {
                onclick: move |_| count += 1,
                "Click to update Canvas"
            }
        }
    }
}
