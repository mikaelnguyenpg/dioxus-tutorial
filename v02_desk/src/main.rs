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
            h2 { "2.A Canvas Test (HTML5 Canvas)" }
            CanvasExample {}
        }

        div { class: "test-section",
            h2 { "2.B Canvas Test 2 (HTML5 Canvas Points)" }
            CanvasExample2 {}
        }

        div { class: "test-section",
            h2 { "2.C Canvas Test 3 (HTML5 Canvas Points Continuously)" }
            CanvasExample3 {}
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

#[component]
fn CanvasExample2() -> Element {
    let mut points_count = use_signal(|| 1000);
    let mut seed = use_signal(|| 0); // Dùng để trigger regenerate

    use_effect(move || {
        let count = points_count();
        let random_seed = seed(); // trigger khi thay đổi

        let script = format!(
            r#"
            const canvas = document.getElementById('points-canvas');
            if (!canvas) return;
            const ctx = canvas.getContext('2d');
            
            // Clear canvas
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            
            // Background
            ctx.fillStyle = '#0f172a';
            ctx.fillRect(0, 0, 600, 400);
            
            // Vẽ 1000 điểm ngẫu nhiên
            ctx.fillStyle = '#60a5fa';
            for (let i = 0; i < {}; i++) {{
                const x = Math.random() * 580 + 10;
                const y = Math.random() * 380 + 10;
                const radius = Math.random() * 3 + 1.5;
                
                ctx.beginPath();
                ctx.arc(x, y, radius, 0, Math.PI * 2);
                ctx.fill();
            }}
            
            // Tiêu đề
            ctx.fillStyle = '#e2e8f0';
            ctx.font = 'bold 20px Arial';
            ctx.fillText('1000 Random Points', 20, 35);
            
            ctx.font = '16px Arial';
            ctx.fillText('Dioxus Desktop Canvas Demo', 20, 370);
            "#,
            count
        );

        dioxus::document::eval(&script);
    });

    rsx! {
        div {
            h2 { "CanvasExample2: 1000 Random Points" }

            canvas {
                id: "points-canvas",
                width: "600",
                height: "400",
                style: "border: 2px solid #334155; border-radius: 12px; background: #0f172a;"
            }

            div { style: "margin-top: 12px;",
                button {
                    onclick: move |_| {
                        seed.set(seed() + 1); // trigger regenerate
                    },
                    "🔄 Regenerate 1000 Points"
                }

                span { style: "margin-left: 15px; color: #64748b;",
                    "{points_count} points"
                }
            }
        }
    }
}

#[component]
fn CanvasExample3() -> Element {
    let mut frame = use_signal(|| 0u32);

    // Animation loop
    use_effect(move || {
        let current_frame = frame();

        let script = format!(
            r#"
            const canvas = document.getElementById('animation-canvas');
            if (!canvas) return;
            const ctx = canvas.getContext('2d');
            
            const width = canvas.width;
            const height = canvas.height;
            
            // Clear với hiệu ứng mờ nhẹ (trail effect)
            ctx.fillStyle = 'rgba(15, 23, 42, 0.15)';
            ctx.fillRect(0, 0, width, height);
            
            // Vẽ 10000 điểm với vị trí ngẫu nhiên + chuyển động nhẹ
            ctx.fillStyle = '#60a5fa';
            for (let i = 0; i < 10000; i++) {{
                // Tạo vị trí ngẫu nhiên có tính "có hệ thống" theo frame
                const x = (Math.sin(i * 0.1 + {0}) * 200 + width / 2) + (Math.random() * 80 - 40);
                const y = (Math.cos(i * 0.08 + {0} * 1.3) * 140 + height / 2) + (Math.random() * 60 - 30);
                
                const radius = Math.random() * 2.5 + 1.2;
                
                ctx.beginPath();
                ctx.arc(x, y, radius, 0, Math.PI * 2);
                ctx.fill();
            }}
            
            // Tiêu đề
            ctx.fillStyle = '#e2e8f0';
            ctx.font = 'bold 22px Arial';
            ctx.fillText('CanvasExample3 - 1000 Animated Points', 20, 40);
            
            ctx.font = '16px Arial';
            ctx.fillStyle = '#94a3b8';
            ctx.fillText('Real-time animation with Dioxus Desktop', 20, 370);
            "#,
            current_frame
        );

        dioxus::document::eval(&script);

        // Tăng frame để tạo animation
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(30)).await; // ~33 FPS
            frame.set(current_frame + 1);
        });
    });

    rsx! {
        div {
            h2 { "CanvasExample3: 10000 Animated Points" }

            canvas {
                id: "animation-canvas",
                width: "600",
                height: "400",
                style: "border: 2px solid #334155; border-radius: 12px; background: #0f172a;"
            }

            p { style: "color: #64748b; margin-top: 8px;",
                "Đang chạy animation realtime (~33 FPS)"
            }
        }
    }
}
