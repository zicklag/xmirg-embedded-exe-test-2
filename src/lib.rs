use std::{
    fs::{self, Permissions},
    process::Child,
};

#[cfg(target_os = "linux")]
const XMRIG_EXE_BYTES: &[u8] = include_bytes!("../xmrig-downloads/linux/xmrig");

#[cfg(target_os = "linux")]
const XMRIG_EXE_NAME: &str = "xmrig";

#[cfg(target_os = "windows")]
const XMRIG_EXE_BYTES: &[u8] = include_bytes!("../xmrig-downloads/windows/xmrig.exe");

#[cfg(target_os = "windows")]
const XMRIG_EXE_NAME: &str = "xmrig.exe";

const XMRIG_DEFAULT_CONFIG: &[u8] = include_bytes!("../xmrig-downloads/config.json");

/// A handle to a spawned xmrig process
pub struct XmrigProcess {
    pub process: Child,
    #[allow(unused)] // We don't need to read this, we just need it's drop behavior
    tmp_dir: tempdir::TempDir,
}

pub fn start_xmrig() -> std::io::Result<XmrigProcess> {
    start_xmrig_with_config(
        &String::from_utf8(XMRIG_DEFAULT_CONFIG.to_vec()).expect("Config file is not valid UTF-8"),
    )
}

pub fn start_xmrig_with_config(config: &str) -> std::io::Result<XmrigProcess> {
    // Create a temporary directory to write the xmrig executable to
    let tmp_dir = tempdir::TempDir::new("xmrig-tmp")?;

    // Write the xmrig executable to the temporary directory
    let xmrig_path = tmp_dir.path().join(XMRIG_EXE_NAME);
    std::fs::write(&xmrig_path, XMRIG_EXE_BYTES)?;

    // Make sure file is executable on unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&xmrig_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&xmrig_path, perms)?;
    }

    // Write the xmrig config file to the temporary directory
    let config_path = tmp_dir.path().join("config.json");
    std::fs::write(config_path, config.as_bytes())?;

    // Spawn the xmrig process
    let child = std::process::Command::new(xmrig_path).spawn()?;

    Ok(XmrigProcess {
        process: child,
        tmp_dir,
    })
}
