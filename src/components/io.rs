//! Browser file import/export.
//!
//! Both directions are one JS shim each — a hidden `<input type="file">` for
//! reading and an object-URL anchor for writing — shared by characters and
//! quirk packs so neither format carries its own copy of the glue.

use base64::prelude::*;
use dioxus::prelude::document;

use arrata_lib::{Character, Quirk};

/// Opens a file picker and returns the raw bytes of each chosen file.
///
/// Empty if the user cancels or the selection can't be decoded.
async fn pick_files(accept: &str, multiple: bool) -> Vec<Vec<u8>> {
    let js = format!(
        r#"
        var input = document.createElement("input");
        input.type = "file";
        input.accept = "{accept}";
        input.multiple = {multiple};
        input.onchange = async function(e) {{
            var results = [];
            for (var i = 0; i < e.target.files.length; i++) {{
                var bytes = new Uint8Array(await e.target.files[i].arrayBuffer());
                var chunks = "";
                for (var j = 0; j < bytes.length; j += 8192) {{
                    chunks += String.fromCharCode.apply(null, bytes.subarray(j, j + 8192));
                }}
                results.push(btoa(chunks));
            }}
            dioxus.send(results);
        }};
        input.click();
    "#
    );

    let mut eval = document::eval(&js);
    let Ok(encoded) = eval.recv::<Vec<String>>().await else {
        return Vec::new();
    };

    encoded
        .iter()
        .filter_map(|b64| BASE64_STANDARD.decode(b64.as_bytes()).ok())
        .collect()
}

/// Triggers a browser download of `bytes` as `filename`.
fn download(filename: &str, bytes: &[u8]) {
    let b64 = BASE64_STANDARD.encode(bytes);
    let js = format!(
        r#"var b=atob("{b64}");var u=new Uint8Array(b.length);for(var i=0;i<b.length;i++)u[i]=b.charCodeAt(i);var blob=new Blob([u],{{type:"application/octet-stream"}});var a=document.createElement("a");a.href=URL.createObjectURL(blob);a.download="{filename}";a.click();URL.revokeObjectURL(a.href);"#
    );
    let _ = document::eval(&js);
}

/// Picks a single `.arrata` file. `None` if cancelled or invalid.
pub async fn pick_character() -> Option<Character> {
    let files = pick_files(".arrata", false).await;
    bitcode::decode(files.first()?).ok()
}

/// Downloads `character` as `<name>.arrata`.
pub fn download_character(character: &Character) {
    download(
        &format!("{}.arrata", character.name),
        &bitcode::encode(character),
    );
}

/// Picks any number of `.quirks` files and concatenates their contents.
pub async fn pick_quirks() -> Vec<Quirk> {
    pick_files(".quirks", true)
        .await
        .iter()
        .filter_map(|bytes| bitcode::decode::<Vec<Quirk>>(bytes).ok())
        .flatten()
        .collect()
}

/// Downloads `quirks` as `quirks.quirks`.
pub fn download_quirks(quirks: &[Quirk]) {
    download("quirks.quirks", &bitcode::encode(quirks));
}
