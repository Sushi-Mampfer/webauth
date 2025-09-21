use std::{
    cell::RefCell,
    env::var,
    ffi::OsString,
    ops::Deref,
    os::windows::ffi::OsStrExt,
    sync::{Arc, Mutex},
};

use windows::Win32::{
    Foundation::{E_NOTIMPL, GENERIC_WRITE, S_FALSE},
    Storage::FileSystem::{
        CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_WRITE, OPEN_EXISTING, WriteFile,
    },
    System::Com::CoTaskMemAlloc,
    UI::Shell::{
        CPFG_CREDENTIAL_PROVIDER_LABEL, CPFG_CREDENTIAL_PROVIDER_LOGO,
        CPFG_STANDALONE_SUBMIT_BUTTON, CPFT_LARGE_TEXT, CPFT_SUBMIT_BUTTON, CPFT_TILE_IMAGE,
        CPUS_LOGON, CPUS_UNLOCK_WORKSTATION, CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR,
        CREDENTIAL_PROVIDER_USAGE_SCENARIO, ICredentialProvider, ICredentialProvider_Impl,
        ICredentialProviderEvents,
    },
};
use windows_core::{PCWSTR, PWSTR, Ref, implement};

use crate::CredentialProviderCredential;

#[implement(ICredentialProvider)]
#[derive(Clone)]
pub struct CredentialProvider {
    event: Arc<Mutex<Option<ICredentialProviderEvents>>>,
    context: Arc<Mutex<Option<usize>>>,
    computername: Arc<String>,
    username: Arc<Mutex<Option<String>>>,
    password: Arc<Mutex<Option<String>>>,
}

impl CredentialProvider {
    pub fn new() -> Option<Self> {
        Some(Self {
            event: Arc::new(Mutex::new(None)),
            context: Arc::new(Mutex::new(None)),
            computername: Arc::new(var("computername").ok()?),
            username: Arc::new(Mutex::new(None)),
            password: Arc::new(Mutex::new(None)),
        })
    }

    pub fn update(&self, username: String, passwd: String) {
        *self.username.lock().unwrap() = Some(username);
        *self.password.lock().unwrap() = Some(passwd);
        let event = self.event.lock().unwrap().clone();
        let context = self.context.lock().unwrap().clone();
        if event.is_some() && context.is_some() {
            unsafe {
                let _ = event.unwrap().CredentialsChanged(context.unwrap());
            }
        }
    }
}

impl ICredentialProvider_Impl for CredentialProvider_Impl {
    fn SetUsageScenario(
        &self,
        cpus: CREDENTIAL_PROVIDER_USAGE_SCENARIO,
        dwflags: u32,
    ) -> windows_core::Result<()> {
        match cpus {
            CPUS_LOGON | CPUS_UNLOCK_WORKSTATION => Ok(()),
            _ => Err(E_NOTIMPL.into()),
        }
    }

    fn SetSerialization(
        &self,
        pcpcs: *const windows::Win32::UI::Shell::CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION,
    ) -> windows_core::Result<()> {
        Err(S_FALSE.into())
    }

    fn Advise(
        &self,
        pcpe: windows_core::Ref<windows::Win32::UI::Shell::ICredentialProviderEvents>,
        upadvisecontext: usize,
    ) -> windows_core::Result<()> {
        *self.event.lock().unwrap() = pcpe.cloned();
        *self.context.lock().unwrap() = Some(upadvisecontext);
        Ok(())
    }

    fn UnAdvise(&self) -> windows_core::Result<()> {
        *self.event.lock().unwrap() = None;
        Ok(())
    }

    fn GetFieldDescriptorCount(&self) -> windows_core::Result<u32> {
        Ok(0)
    }

    fn GetFieldDescriptorAt(
        &self,
        dwindex: u32,
    ) -> windows_core::Result<*mut windows::Win32::UI::Shell::CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR>
    {
        return Err(E_NOTIMPL.into());
    }

    fn GetCredentialCount(
        &self,
        pdwcount: *mut u32,
        pdwdefault: *mut u32,
        pbautologonwithdefault: *mut windows_core::BOOL,
    ) -> windows_core::Result<()> {
        if let Some(_) = *self.username.lock().unwrap() {
            unsafe {
                *pdwcount = 1;
                *pdwdefault = 0;
                *pbautologonwithdefault = true.into();
            }
        } else {
            unsafe {
                *pdwcount = 0;
                *pdwdefault = 0;
                *pbautologonwithdefault = true.into();
            }
        }
        Ok(())
    }

    fn GetCredentialAt(
        &self,
        dwindex: u32,
    ) -> windows_core::Result<windows::Win32::UI::Shell::ICredentialProviderCredential> {
        let username = self.username.lock().unwrap().clone();
        let password = self.password.lock().unwrap().clone();
        if password.is_some() && username.is_some() {
            Ok(CredentialProviderCredential::CredentialProviderCredential {
                computername: (*self.computername).clone(),
                username: username.unwrap(),
                password: password.unwrap(),
            }
            .into())
        } else {
            Err(E_NOTIMPL.into())
        }
    }
}
