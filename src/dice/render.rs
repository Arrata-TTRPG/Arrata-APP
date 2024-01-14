use dioxus::prelude::*;

//use crate::character::structs::*;

pub fn render_rolls(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "z-10 fixed text-center max-w-[80%] w-96 h-48 border text-white border-white bg-slate-800 m-auto left-0 right-0 top-0 bottom-0 rounded-lg",
            div {
                "Outcome:"
            }
        }
    })
}
