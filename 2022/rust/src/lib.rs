use std::{
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};

pub mod extra_itertools;

#[macro_export]
macro_rules! open {
    ($file_name:expr) => {{
        $crate::open_file(file!(), $file_name)
    }};
}

#[inline]
pub fn open_file(src_file: &str, file_name: impl AsRef<Path>) -> Result<File> {
    File::open(resolve_file(src_file, file_name)?).context("Can't open file")
}

fn resolve_file(src_file: &str, file_name: impl AsRef<Path>) -> Result<PathBuf> {
    let mut task_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if !task_data_dir.pop() {
        bail!(
            "Something wrong with build path {}",
            task_data_dir.display()
        );
    }
    task_data_dir.push("data");
    task_data_dir.push(
        Path::new(src_file)
            .file_stem()
            .context("Can't determine AoC day")?,
    );
    if !task_data_dir.is_dir() {
        bail!("Task data dir not found at {}", task_data_dir.display());
    }
    task_data_dir.push(file_name);
    Ok(task_data_dir)
}
