use std::{
    collections::HashMap,
    ffi::OsString,
    os::windows::ffi::OsStrExt,
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::Duration,
};

use axum::{Router, extract::Query, response::IntoResponse, routing::get};
use tokio::task::spawn_blocking;
use windows::Win32::{
    Foundation::{ERROR_BROKEN_PIPE, ERROR_NO_DATA, GetLastError, HANDLE, INVALID_HANDLE_VALUE},
    Storage::FileSystem::{PIPE_ACCESS_OUTBOUND, WriteFile},
    System::Pipes::{
        ConnectNamedPipe, CreateNamedPipeW, DisconnectNamedPipe, PIPE_READMODE_BYTE,
        PIPE_TYPE_BYTE, PIPE_WAIT,
    },
};
use windows_core::PCWSTR;

#[derive(Debug)]
struct SafeHandle(HANDLE);

unsafe impl Send for SafeHandle {}
unsafe impl Sync for SafeHandle {}

static PIPE: OnceLock<SafeHandle> = OnceLock::new();
static CONNECTED: AtomicBool = AtomicBool::new(false);

#[tokio::main]
async fn main() {
    let wide: Vec<u16> = OsString::from("\\\\.\\pipe\\webauth")
        .encode_wide()
        .chain([0])
        .collect();
    unsafe {
        let pipe = CreateNamedPipeW(
            PCWSTR(wide.as_ptr()),
            PIPE_ACCESS_OUTBOUND,
            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
            1,
            512,
            512,
            0,
            None,
        );
        if pipe == INVALID_HANDLE_VALUE {
            panic!("invalid pipe")
        }
        PIPE.set(SafeHandle(pipe)).unwrap();

        spawn_blocking(|| {
            loop {
                if CONNECTED.load(Ordering::Relaxed) {
                    let result = WriteFile(PIPE.get().unwrap().0, Some(b"ping"), None, None);
                    match result {
                        Ok(_) => (),
                        _ => match GetLastError() {
                            ERROR_BROKEN_PIPE | ERROR_NO_DATA => {
                                let _ = DisconnectNamedPipe(PIPE.get().unwrap().0);
                                CONNECTED.store(false, Ordering::Relaxed);
                            }
                            e => {
                                println!("Unexpected error: {}", e.0);
                                break;
                            }
                        },
                    }
                    sleep(Duration::from_secs(1));
                    continue;
                }
                ConnectNamedPipe(PIPE.get().unwrap().0, None).unwrap();
                CONNECTED.store(true, Ordering::Relaxed);
            }
        });
    }

    let app = Router::new().route("/", get(login));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4242").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn login(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    if !CONNECTED.load(Ordering::Relaxed) {
        return "no connection".to_string();
    }
    if let (Some(username), Some(passwd)) = (params.get("username"), params.get("passwd")) {
        let message = format!("{};{}", username, passwd);
        let result =
            unsafe { WriteFile(PIPE.get().unwrap().0, Some(message.as_bytes()), None, None) };
        match result {
            Ok(_) => return "success".to_string(),
            _ => match unsafe { GetLastError() } {
                ERROR_BROKEN_PIPE | ERROR_NO_DATA => {
                    unsafe {
                        let _ = DisconnectNamedPipe(PIPE.get().unwrap().0);
                    };
                    CONNECTED.store(false, Ordering::Relaxed);
                    return "no connection".to_string();
                }
                e => {
                    let err = format!("Unexpected error: {}", e.0);
                    return err;
                }
            },
        }
    }
    "invalid params".to_string()
}
