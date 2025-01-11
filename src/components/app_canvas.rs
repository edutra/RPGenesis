use crate::components::canvas_cell::CanvasCell;
use leptos::prelude::*;
use leptos_use::{use_window_size, UseWindowSizeReturn};
#[component]
pub fn AppCanvas(cell_size: i32) -> impl IntoView {
    fn setCanvasStyle(cols: i32, rows: i32) -> String {
        return format!("width: 100%; height: 100vh; background-color: white; display: grid; grid-template-columns: repeat({}, 1fr); grid-template-rows: repeat({}, 1fr);", cols, rows);
    }
    let UseWindowSizeReturn { width, height } = use_window_size();
    let columns_repeat: i32 = Memo::new(move |_| width.get() / cell_size as f64).get() as i32;
    let rows_repeat: i32 = Memo::new(move |_| height.get() / cell_size as f64).get() as i32;

    view! {
        <div style=setCanvasStyle(columns_repeat, rows_repeat)>
        {(0..(columns_repeat * rows_repeat) as i32).map(|_| view! {
            <CanvasCell />
        }
        ).collect::<Vec<_>>()}
        </div>

    }
}
