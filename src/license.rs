use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

const PRODUCT: &str = "function-flow-canvas";
const BILLING_BASE: &str = "https://api.sociobot.in/api/v1";
const DAY: u64 = 86_400;

#[derive(Debug, Serialize, Deserialize)]
struct Cache {
    token: String,
    valid: bool,
    verified_at: u64,
}

#[derive(Debug, Deserialize)]
struct Verdict {
    valid: bool,
    reason: String,
}

pub fn require_for_depth(depth: u8, explicit: Option<String>) -> Result<(), String> {
    if depth <= 2 {
        return Ok(());
    }
    let cache = read_cache();
    let token = explicit
        .or_else(|| std::env::var("FFC_LICENSE").ok())
        .or_else(|| cache.as_ref().map(|saved| saved.token.clone()))
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            format!(
                "depth {depth} requires the Pathfinder unlock; buy once at {BILLING_BASE}/products/{PRODUCT}/checkout, then set FFC_LICENSE"
            )
        })?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    if cache.as_ref().is_some_and(|saved| {
        saved.token == token && saved.valid && now.saturating_sub(saved.verified_at) < DAY
    }) {
        return Ok(());
    }

    let url = format!(
        "{BILLING_BASE}/products/{PRODUCT}/verify?license={}",
        percent_encode(&token)
    );
    let response = ureq::get(&url).call();
    match response {
        Ok(mut response) => {
            let verdict: Verdict = response
                .body_mut()
                .read_json()
                .map_err(|error| format!("license response could not be read: {error}"))?;
            write_cache(&Cache {
                token,
                valid: verdict.valid,
                verified_at: now,
            });
            if verdict.valid {
                Ok(())
            } else {
                Err(format!(
                    "license is not active ({}); restore or buy at {BILLING_BASE}/products/{PRODUCT}/checkout",
                    verdict.reason
                ))
            }
        }
        Err(error)
            if cache
                .as_ref()
                .is_some_and(|saved| saved.token == token && saved.valid) =>
        {
            eprintln!("ffc: offline; using the last valid Pathfinder license ({error})");
            Ok(())
        }
        Err(error) => Err(format!(
            "could not verify the Pathfinder license ({error}); connect once or use depth 1–2"
        )),
    }
}

fn cache_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))?;
    Some(base.join(PRODUCT).join("license.json"))
}

fn read_cache() -> Option<Cache> {
    let bytes = fs::read(cache_path()?).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn write_cache(cache: &Cache) {
    let Some(path) = cache_path() else { return };
    let _ = write_cache_to(&path, cache);
}

fn write_cache_to(path: &Path, cache: &Cache) -> std::io::Result<()> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    fs::create_dir_all(parent)?;
    #[cfg(unix)]
    fs::set_permissions(parent, fs::Permissions::from_mode(0o700))?;

    let bytes = serde_json::to_vec(cache).map_err(std::io::Error::other)?;
    let mut options = fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    #[cfg(unix)]
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    file.write_all(&bytes)
}

fn percent_encode(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (byte as char).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn free_depth_never_requires_network() {
        assert!(require_for_depth(2, None).is_ok());
    }

    #[test]
    fn token_is_query_safe() {
        assert_eq!(percent_encode("a+b/c="), "a%2Bb%2Fc%3D");
    }

    #[cfg(unix)]
    #[test]
    fn cache_file_and_directory_are_user_only() {
        use std::os::unix::fs::PermissionsExt;

        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("private").join("license.json");
        write_cache_to(
            &path,
            &Cache {
                token: "secret-token".into(),
                valid: true,
                verified_at: 1,
            },
        )
        .unwrap();
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o600
        );
        assert_eq!(
            fs::metadata(path.parent().unwrap())
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o700
        );
    }
}
