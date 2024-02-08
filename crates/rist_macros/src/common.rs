use std::{
    env, fs::File, path::{Path, PathBuf}
};

use proc_macro2::Span;

use crate::instruction_table::InstructionTable;

pub(crate) fn get_instruction_table() -> syn::Result<InstructionTable> {
    let instruction_table = resolve_path("instr_dict.yaml", Span::call_site())?;
    let instruction_table =
        File::open(instruction_table).map_err(|error| syn::Error::new(Span::call_site(), error))?;

    serde_yaml::from_reader::<_, InstructionTable>(instruction_table)
        .map_err(|error| syn::Error::new(Span::call_site(), error))
}

// Based on https://github.com/launchbadge/sqlx/blob/main/sqlx-macros-core/src/common.rs
pub(crate) fn resolve_path(path: impl AsRef<Path>, err_span: Span) -> syn::Result<PathBuf> {
    let path = path.as_ref();

    if path.is_absolute() {
        return Err(syn::Error::new(
            err_span,
            "absolute paths will only work on the current machine",
        ));
    }

    let base_dir = env::var("CARGO_MANIFEST_DIR").map_err(|_| {
        syn::Error::new(err_span, "CARGO_MANIFEST_DIR is not set; please use Cargo to build")
    })?;
    let base_dir_path = Path::new(&base_dir);

    Ok(base_dir_path.join(path))
}
