use std::{
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};

pub fn data_dir() -> Result<PathBuf> {
    let mut task_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if !task_data_dir.pop() {
        bail!(
            "Something wrong with build path {}",
            task_data_dir.display()
        );
    }
    task_data_dir.push("data");
    task_data_dir.push(
        std::env::current_exe()
            .context("Can't determine current executable")?
            .file_name()
            .context("Can't determine current executable's name")?,
    );
    if !task_data_dir.is_dir() {
        bail!("Task data dir not found at {}", task_data_dir.display());
    }
    Ok(task_data_dir)
}

pub fn open(file_name: impl AsRef<Path>) -> Result<File> {
    let mut path = data_dir()?;
    path.push(file_name);
    File::open(path).context("Can't open file")
}
