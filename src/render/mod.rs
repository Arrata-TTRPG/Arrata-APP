pub mod entrypoint;
pub use entrypoint::App;

pub(crate) mod character;
pub(crate) use character::*;

pub(crate) mod sidebar;
pub(crate) use sidebar::{CharacterSidebar, SidebarToggle};

pub(crate) mod rolls;
pub(crate) use rolls::RenderRolls;

pub(crate) mod quirks;
pub(crate) use quirks::RenderQuirks;

pub(crate) mod premade_quirks;

pub(crate) mod stats;
pub(crate) use stats::RenderStats;

pub(crate) mod combat;
pub(crate) use combat::RenderCombat;

use arrata_lib::character::Character;
use base64::prelude::*;
use dioxus::prelude::document;

/// Opens a file picker for `.arrata` files and decodes the result.
/// Returns `None` if the user cancels or the file is invalid.
pub(crate) async fn pick_character_file() -> Option<Character> {
    let js = r#"
        var input = document.createElement("input");
        input.type = "file";
        input.accept = ".arrata";
        input.onchange = async function(e) {
            var file = e.target.files[0];
            if (!file) return;
            var buf = await file.arrayBuffer();
            var bytes = new Uint8Array(buf);
            var b64 = "";
            for (var i = 0; i < bytes.length; i += 8192) {
                b64 += String.fromCharCode.apply(null, bytes.subarray(i, i + 8192));
            }
            dioxus.send(btoa(b64));
        };
        input.click();
    "#;
    let mut eval = document::eval(js);
    let val = eval.recv::<String>().await.ok()?;
    let bytes = BASE64_STANDARD.decode(val.as_bytes()).ok()?;

    if let Ok(c) = bitcode::decode::<Character>(&bytes) {
        Some(c)
    } else {
        log::error!("Failed to decode .arrata file");
        None
    }
}

/// Encodes `character` and triggers a browser download as `<name>.arrata`.
pub(crate) fn download_character(character: &Character) {
    let bytes = bitcode::encode(character);
    let b64 = BASE64_STANDARD.encode(&bytes);
    let filename = format!("{}.arrata", character.name);
    let js = format!(
        r#"var b=atob("{b64}");var u=new Uint8Array(b.length);for(var i=0;i<b.length;i++)u[i]=b.charCodeAt(i);var blob=new Blob([u],{{type:"application/octet-stream"}});var a=document.createElement("a");a.href=URL.createObjectURL(blob);a.download="{filename}";a.click();URL.revokeObjectURL(a.href);"#
    );
    let _ = document::eval(&js);
}

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
