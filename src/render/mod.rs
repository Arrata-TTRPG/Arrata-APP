pub mod entrypoint;
pub use entrypoint::App;

pub(crate) mod character;
pub(crate) use character::*;

pub(crate) mod rolls;
pub(crate) use rolls::RenderRolls;

pub(crate) mod quirks;
pub(crate) use quirks::RenderQuirks;

pub(crate) mod premade_quirks;

pub(crate) mod stats;
pub(crate) use stats::RenderStats;

pub(crate) mod combat;
pub(crate) use combat::RenderCombat;

/// Returns JS that auto-sizes a textarea to its content without disturbing scroll position.
/// Pass `initial: true` for `onmounted` (uses setTimeout to wait for layout),
/// `initial: false` for `oninput`.
pub(crate) fn auto_resize_js(id: &str, initial: bool) -> String {
    let core = format!(
        "var t=document.getElementById('{id}');if(t){{var s=window.scrollY;t.style.height='auto';t.style.height=t.scrollHeight+'px';window.scrollTo(0,s);}}"
    );
    if initial {
        format!("setTimeout(function(){{{core}}},0);")
    } else {
        core
    }
}

