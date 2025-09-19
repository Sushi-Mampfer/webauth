use std::{cell::RefCell, env::var, ffi::OsString, ops::Deref, os::windows::ffi::OsStrExt};

use windows::Win32::{
    Foundation::{E_NOTIMPL, S_FALSE},
    System::Com::CoTaskMemAlloc,
    UI::Shell::{
        CPFG_CREDENTIAL_PROVIDER_LABEL, CPFG_CREDENTIAL_PROVIDER_LOGO,
        CPFG_STANDALONE_SUBMIT_BUTTON, CPFT_LARGE_TEXT, CPFT_SUBMIT_BUTTON, CPFT_TILE_IMAGE,
        CPUS_LOGON, CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR, CREDENTIAL_PROVIDER_USAGE_SCENARIO,
        ICredentialProvider, ICredentialProvider_Impl, ICredentialProviderEvents,
    },
};
use windows_core::{PWSTR, Ref, implement};

use crate::CredentialProviderCredential;

#[implement(ICredentialProvider)]
pub struct CredentialProvider {
    event: RefCell<Option<ICredentialProviderEvents>>,
    computername: String,
    username: RefCell<Option<String>>,
    password: RefCell<Option<String>>,
}

impl CredentialProvider {
    pub fn new() -> Option<Self> {
        Some(Self {
            event: RefCell::new(None),
            computername: var("computername").ok()?,
            username: RefCell::new(None),
            password: RefCell::new(None),
        })
    }
}

impl ICredentialProvider_Impl for CredentialProvider_Impl {
    fn SetUsageScenario(
        &self,
        cpus: CREDENTIAL_PROVIDER_USAGE_SCENARIO,
        dwflags: u32,
    ) -> windows_core::Result<()> {
        match cpus {
            CPUS_LOGON => Ok(()),
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
        *self.event.borrow_mut() = pcpe.cloned();
        Ok(())
    }

    fn UnAdvise(&self) -> windows_core::Result<()> {
        *self.event.borrow_mut() = None;
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
        if let Some(_) = *self.username.borrow() {
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
        if self.password.borrow().is_some() && self.username.borrow().is_some() {
            Ok(CredentialProviderCredential::CredentialProviderCredential {
                computername: self.computername.clone(),
                username: self.username.borrow().clone().unwrap(),
                password: self.password.borrow().clone().unwrap(),
            }
            .into())
        } else {
            Err(E_NOTIMPL.into())
        }
    }
}
