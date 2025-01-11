use leptos::prelude::*;
#[component]
pub fn CanvasCell() -> impl IntoView {
    fn setStyle(isActive: bool) -> String {
        if isActive{
            return "border: 1px solid red; display: flex; justify-content: center; align-items: center; aspect-ratio: 1 / 1;".to_string();
        }
        return "border: 1px solid green; display: flex; justify-content: center; align-items: center; aspect-ratio: 1 / 1;".to_string();
    }
    let (use_is_active, set_is_active) = signal(false);
    view! {
        <div on:click=move |_| {
            set_is_active.set(!use_is_active.get());
        } style="border: 1px solid #d6d6d6; display: flex; justify-content: center; align-items: center; aspect-ratio: 1 / 1;"
        style:border=move || {
            if use_is_active.get() {
                return "1px solid red";
            }
            return "1px solid green";
        }>
        </div>

    }
}
