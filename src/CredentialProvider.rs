use std::{ffi::OsString, os::windows::ffi::OsStrExt};

use windows::Win32::{
    Foundation::{E_NOTIMPL, S_FALSE},
    System::Com::CoTaskMemAlloc,
    UI::Shell::{
        CPFG_CREDENTIAL_PROVIDER_LABEL, CPFG_CREDENTIAL_PROVIDER_LOGO,
        CPFG_STANDALONE_SUBMIT_BUTTON, CPFT_LARGE_TEXT, CPFT_SUBMIT_BUTTON, CPFT_TILE_IMAGE,
        CPUS_LOGON, CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR, CREDENTIAL_PROVIDER_USAGE_SCENARIO,
        ICredentialProvider, ICredentialProvider_Impl,
    },
};
use windows_core::{PWSTR, implement};

use crate::CredentialProviderCredential;

#[implement(ICredentialProvider)]
pub struct CredentialProvider();

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
        Err(E_NOTIMPL.into())
    }

    fn UnAdvise(&self) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetFieldDescriptorCount(&self) -> windows_core::Result<u32> {
        Ok(3)
    }

    fn GetFieldDescriptorAt(
        &self,
        dwindex: u32,
    ) -> windows_core::Result<*mut windows::Win32::UI::Shell::CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR>
    {
        unsafe {
            let mem: *mut CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR =
                CoTaskMemAlloc(size_of::<CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR>())
                    as *mut CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR;
            let field = match dwindex {
                0 => {
                    let wide: Vec<u16> = OsString::from("Icon").encode_wide().chain([0]).collect();
                    let size = wide.len() * size_of::<u16>();
                    let label = CoTaskMemAlloc(size) as *mut u16;
                    label.copy_from_nonoverlapping(wide.as_ptr(), wide.len());

                    CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
                        dwFieldID: 0,
                        cpft: CPFT_TILE_IMAGE,
                        pszLabel: PWSTR(label),
                        guidFieldType: CPFG_CREDENTIAL_PROVIDER_LOGO,
                    }
                }
                1 => {
                    let wide: Vec<u16> = OsString::from("Label").encode_wide().chain([0]).collect();
                    let size = wide.len() * size_of::<u16>();
                    let label = CoTaskMemAlloc(size) as *mut u16;
                    label.copy_from_nonoverlapping(wide.as_ptr(), wide.len());

                    CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
                        dwFieldID: 1,
                        cpft: CPFT_LARGE_TEXT,
                        pszLabel: PWSTR(label),
                        guidFieldType: CPFG_CREDENTIAL_PROVIDER_LABEL,
                    }
                }
                2 => {
                    let wide: Vec<u16> =
                        OsString::from("Submit").encode_wide().chain([0]).collect();
                    let size = wide.len() * size_of::<u16>();
                    let label = CoTaskMemAlloc(size) as *mut u16;
                    label.copy_from_nonoverlapping(wide.as_ptr(), wide.len());

                    CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
                        dwFieldID: 2,
                        cpft: CPFT_SUBMIT_BUTTON,
                        pszLabel: PWSTR(label),
                        guidFieldType: CPFG_STANDALONE_SUBMIT_BUTTON,
                    }
                }
                _ => return Err(E_NOTIMPL.into()),
            };

            mem.write(field);

            Ok(mem)
        }
    }

    fn GetCredentialCount(
        &self,
        pdwcount: *mut u32,
        pdwdefault: *mut u32,
        pbautologonwithdefault: *mut windows_core::BOOL,
    ) -> windows_core::Result<()> {
        unsafe {
            *pdwcount = 1;
            *pdwdefault = 0;
            *pbautologonwithdefault = false.into();
        }
        Ok(())
    }

    fn GetCredentialAt(
        &self,
        dwindex: u32,
    ) -> windows_core::Result<windows::Win32::UI::Shell::ICredentialProviderCredential> {
        Ok(CredentialProviderCredential::CredentialProviderCredential().into())
    }
}
