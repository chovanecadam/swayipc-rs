use async_io::Async;
use async_pidfd::AsyncPidFd;
use futures_lite::{future, AsyncReadExt};
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use swayipc_types::{Error, Fallible};

pub async fn get_socketpath() -> Fallible<PathBuf> {
    env::var("I3SOCK")
        .or_else(|_| env::var("SWAYSOCK"))
        .or_else(|_| future::block_on(spawn("i3")))
        .or_else(|_| future::block_on(spawn("sway")))
        .map_err(|_| Error::ConnectionError)
        .map(PathBuf::from)
}

async fn spawn(wm: &str) -> Fallible<String> {
    let mut child = Command::new(wm)
        .arg("--get-socketpath")
        .stdout(Stdio::piped())
        .spawn()?;
    let mut buf = String::new();
    if let Some(stdout) = child.stdout.take() {
        Async::new(stdout)?.read_to_string(&mut buf).await?;
        buf.pop();
    }
    AsyncPidFd::from_pid(child.id() as i32)?.wait().await?;
    Ok(buf)
}
