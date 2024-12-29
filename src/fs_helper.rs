use std::fs;
use std::io::Result;
use std::path::Path;

#[cfg(windows)]
use mslnk::{MSLinkError, ShellLink};

#[cfg(target_os = "linux")]
fn create_desktop_file(source_path: &Path, target_path: &Path) -> Result<()> {
    let content = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name={}\n\
         Exec={}\n\
         Terminal=false\n\
         Categories=Application;",
        source_path.file_name().unwrap().to_string_lossy(),
        source_path.display()
    );
    fs::write(target_path, content)
}

#[cfg(windows)]
pub fn create_windows_shortcut(source_path: &Path, target_path: &Path) -> Result<(), MSLinkError> {
    let sl = ShellLink::new(source_path)?;
    sl.create_lnk(target_path)?;

    Ok(())
}
