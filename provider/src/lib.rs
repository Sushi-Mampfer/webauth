use std::ffi::{OsStr, OsString};
use std::fs::OpenOptions;
use std::io::Write;
use std::os::windows::ffi::OsStrExt;
use std::thread::{sleep, spawn};
use std::time::Duration;
use std::{ffi, mem, ptr};

use windows::Wdk::System::SystemServices::DbgPrint;
use windows::Win32::Foundation::{
    CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION, CloseHandle, E_INVALIDARG, E_NOINTERFACE,
    E_NOTIMPL, E_POINTER, GENERIC_READ, GENERIC_WRITE, GetLastError, HANDLE, INVALID_HANDLE_VALUE,
    S_OK,
};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_WRITE, FILE_SHARE_NONE, FILE_SHARE_READ,
    FILE_SHARE_WRITE, OPEN_EXISTING, ReadFile, WriteFile,
};
use windows::Win32::System::Com::{IClassFactory, IClassFactory_Impl};
use windows::Win32::System::Diagnostics::Debug::OutputDebugStringW;
use windows::Win32::UI::Shell::ICredentialProvider;
use windows_core::{BOOL, GUID, HRESULT, IUnknown, Interface, PCWSTR, PWSTR, implement};

#[allow(non_snake_case)]
mod CredentialProvider;
#[allow(non_snake_case)]
mod CredentialProviderCredential;

#[unsafe(no_mangle)]
extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut ffi::c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }

    unsafe { *ppv = ptr::null_mut() };

    if rclsid.is_null() || riid.is_null() {
        return E_INVALIDARG;
    }

    let rclsid = unsafe { *rclsid };
    let riid = unsafe { *riid };

    const CREDENTIAL_PROVIDER_CLSID: GUID = GUID::from_u128(0x7d6836a5_c203_47e2_8fe5_eca9159e7d7e);

    if rclsid != CREDENTIAL_PROVIDER_CLSID {
        return CLASS_E_CLASSNOTAVAILABLE;
    }

    if riid != IClassFactory::IID {
        return E_NOINTERFACE;
    }

    let factory: IClassFactory = ProviderFactory.into();
    unsafe { *ppv = mem::transmute(factory) };
    S_OK
}

#[unsafe(no_mangle)]
extern "system" fn DllCanUnloadNow() -> HRESULT {
    S_OK
}

struct SendableProvider(CredentialProvider::CredentialProvider);

unsafe impl Send for SendableProvider {}
unsafe impl Sync for SendableProvider {}

impl SendableProvider {
    fn update(&self, username: String, password: String) {
        self.0.update(username, password)
    }
}

#[implement(IClassFactory)]
struct ProviderFactory;

impl IClassFactory_Impl for ProviderFactory_Impl {
    fn CreateInstance(
        &self,
        punkouter: windows_core::Ref<'_, IUnknown>,
        riid: *const windows::core::GUID,
        ppvobject: *mut *mut core::ffi::c_void,
    ) -> windows::core::Result<()> {
        if ppvobject.is_null() {
            return Err(E_POINTER.into());
        }
        unsafe { *ppvobject = ptr::null_mut() };
        if riid.is_null() {
            return Err(E_INVALIDARG.into());
        }
        let riid = unsafe { *riid };
        if punkouter.is_some() {
            return Err(CLASS_E_NOAGGREGATION.into());
        }

        if riid != ICredentialProvider::IID {
            return Err(E_NOINTERFACE.into());
        }

        let provider = match CredentialProvider::CredentialProvider::new() {
            Some(p) => p,
            _ => {
                return Err(E_NOINTERFACE.into());
            }
        };

        let provider2 = SendableProvider(provider.clone());
        let provider: ICredentialProvider = provider.into();

        spawn(move || {
            let pipe_name: Vec<u16> = OsString::from("\\\\.\\pipe\\webauth")
                .encode_wide()
                .chain([0])
                .collect();
            let pipe = unsafe {
                CreateFileW(
                    PCWSTR(pipe_name.as_ptr()),
                    GENERIC_READ.0,
                    FILE_SHARE_NONE,
                    None,
                    OPEN_EXISTING,
                    FILE_ATTRIBUTE_NORMAL,
                    None,
                )
            };
            let pipe = match pipe {
                Ok(p) => p,
                Err(e) => {
                    return;
                }
            };
            if pipe == INVALID_HANDLE_VALUE {
                return;
            }
            let mut buffer = [0u8; 512];
            let mut bytes_read = 0u32;
            loop {
                match unsafe { ReadFile(pipe, Some(&mut buffer), Some(&mut bytes_read), None) } {
                    Ok(_) => {
                        let message = &buffer[..bytes_read as usize];
                        if message == b"Q8z!pR3@xY" {
                            sleep(Duration::from_secs(1));
                            continue;
                        }
                        let message = String::from_utf8_lossy(message);
                        let message = message.replace("Q8z!pR3@xY", "");
                        if let Some((username, password)) = message.split_once(';') {
                            provider2.update(username.to_string(), password.to_string())
                        }
                    }
                    Err(_) => sleep(Duration::from_secs(1)),
                }
            }
        });

        unsafe {
            *ppvobject = mem::transmute(provider);
        }
        Ok(())
    }

    fn LockServer(&self, flock: BOOL) -> windows::core::Result<()> {
        let _ = flock;
        Err(E_NOTIMPL.into())
    }
}
