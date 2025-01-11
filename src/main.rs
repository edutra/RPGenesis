mod components;
use leptos::prelude::*;
// use components::btn::MyButton;
use components::app_canvas::AppCanvas;
#[component]
fn App() -> impl IntoView {
    fn setStyle() -> String {
        return "position: relative; background-color: black;".to_string();
    }
    view! {
        <main style=setStyle()>
            <nav style="position: absolute; background-color: blue; bottom: 4rem; left: 35%; width: max-content; border-radius: 2rem;">
                <ul style="display: flex; justify-content: center; align-items: center; gap: 2rem; padding: 2rem;">
                <li style="color: white; font-size: 2rem; text-decoration: none;">
                    Adicionar personagem
                </li>
                <li style="color: white; font-size: 2rem; text-decoration: none;">
                    Adicionar personagem
                </li>
                <li style="color: white; font-size: 2rem; text-decoration: none;">
                    Adicionar personagem
                </li>
                </ul>
            </nav>
        <AppCanvas cell_size=35 />
        </main>

    }
}
fn main() {
    // leptos::mount::mount_to_body(|cx| view! { cx, <App /> });

    leptos::mount::mount_to_body(|| view! { <App /> });
}
