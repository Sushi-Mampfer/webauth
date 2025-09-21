use std::{ffi::OsString, os::windows::ffi::OsStrExt};

use windows::Win32::{
    Foundation::E_NOTIMPL,
    Security::Credentials::{CRED_PACK_PROTECTED_CREDENTIALS, CredPackAuthenticationBufferW},
    System::Com::CoTaskMemAlloc,
    UI::Shell::{
        CPGSR_RETURN_CREDENTIAL_FINISHED, CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION,
        ICredentialProviderCredential, ICredentialProviderCredential_Impl,
    },
};
use windows_core::{GUID, PWSTR, implement};

#[implement(ICredentialProviderCredential)]
pub struct CredentialProviderCredential {
    pub computername: String,
    pub username: String,
    pub password: String,
}

impl CredentialProviderCredential {
    fn user(&self) -> PWSTR {
        unsafe {
            let wide: Vec<u16> =
                OsString::from(format!("{}\\{}", self.computername, self.username))
                    .encode_wide()
                    .chain([0])
                    .collect();
            let size = wide.len() * size_of::<u16>();
            let label = CoTaskMemAlloc(size) as *mut u16;
            label.copy_from_nonoverlapping(wide.as_ptr(), wide.len());
            PWSTR(label)
        }
    }

    fn pasw(&self) -> PWSTR {
        unsafe {
            let wide: Vec<u16> = OsString::from(format!("{}", self.password))
                .encode_wide()
                .chain([0])
                .collect();
            let size = wide.len() * size_of::<u16>();
            let label = CoTaskMemAlloc(size) as *mut u16;
            label.copy_from_nonoverlapping(wide.as_ptr(), wide.len());
            PWSTR(label)
        }
    }
}

impl ICredentialProviderCredential_Impl for CredentialProviderCredential_Impl {
    fn Advise(
        &self,
        _pcpce: windows_core::Ref<windows::Win32::UI::Shell::ICredentialProviderCredentialEvents>,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn UnAdvise(&self) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn SetSelected(&self) -> windows_core::Result<windows_core::BOOL> {
        Ok(false.into())
    }

    fn SetDeselected(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn GetFieldState(
        &self,
        _dwfieldid: u32,
        _pcpfs: *mut windows::Win32::UI::Shell::CREDENTIAL_PROVIDER_FIELD_STATE,
        _pcpfis: *mut windows::Win32::UI::Shell::CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetStringValue(&self, _dwfieldid: u32) -> windows_core::Result<windows_core::PWSTR> {
        Err(E_NOTIMPL.into())
    }

    fn GetBitmapValue(
        &self,
        _dwfieldid: u32,
    ) -> windows_core::Result<windows::Win32::Graphics::Gdi::HBITMAP> {
        Err(E_NOTIMPL.into())
    }

    fn GetCheckboxValue(
        &self,
        _dwfieldidd: u32,
        _pbchecked: *mut windows_core::BOOL,
        _ppszlabel: *mut windows_core::PWSTR,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetSubmitButtonValue(&self, _dwfieldid: u32) -> windows_core::Result<u32> {
        Ok(1)
    }

    fn GetComboBoxValueCount(
        &self,
        _dwfieldid: u32,
        pcitems: *mut u32,
        pdwselecteditem: *mut u32,
    ) -> windows_core::Result<()> {
        unsafe {
            *pcitems = 0;
            *pdwselecteditem = 0;
        }
        Ok(())
    }

    fn GetComboBoxValueAt(
        &self,
        _dwfieldid: u32,
        _dwitem: u32,
    ) -> windows_core::Result<windows_core::PWSTR> {
        Err(E_NOTIMPL.into())
    }

    fn SetStringValue(
        &self,
        _dwfieldid: u32,
        _psz: &windows_core::PCWSTR,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn SetCheckboxValue(
        &self,
        _dwfieldid: u32,
        _bchecked: windows_core::BOOL,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn SetComboBoxSelectedValue(
        &self,
        _dwfieldid: u32,
        _dwselecteditem: u32,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn CommandLinkClicked(&self, _dwfieldid: u32) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetSerialization(
        &self,
        pcpgsr: *mut windows::Win32::UI::Shell::CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE,
        pcpcs: *mut windows::Win32::UI::Shell::CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION,
        _ppszoptionalstatustext: *mut windows_core::PWSTR,
        _pcpsioptionalstatusicon: *mut windows::Win32::UI::Shell::CREDENTIAL_PROVIDER_STATUS_ICON,
    ) -> windows_core::Result<()> {
        unsafe {
            *pcpgsr = CPGSR_RETURN_CREDENTIAL_FINISHED;

            let mut buffer_size = 1024u32;
            let buffer = CoTaskMemAlloc(buffer_size as usize) as *mut u8;

            let result = CredPackAuthenticationBufferW(
                CRED_PACK_PROTECTED_CREDENTIALS,
                self.user(),
                self.pasw(),
                Some(buffer),
                &mut buffer_size,
            );

            if result.is_err() {
                return Err(result.unwrap_err());
            }

            let mem_pcpcs = CoTaskMemAlloc(size_of::<CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION>())
                as *mut CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION;

            mem_pcpcs.write(CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION {
                ulAuthenticationPackage: 0,
                clsidCredentialProvider: GUID::from_u128(0x7d6836a5c20347e28fe5eca9159e7d7e),
                cbSerialization: buffer_size,
                rgbSerialization: buffer,
            });
            *pcpcs = *mem_pcpcs;
        }
        Ok(())
    }

    fn ReportResult(
        &self,
        _ntsstatus: windows::Win32::Foundation::NTSTATUS,
        _ntssubstatus: windows::Win32::Foundation::NTSTATUS,
        _ppszoptionalstatustext: *mut windows_core::PWSTR,
        _pcpsioptionalstatusicon: *mut windows::Win32::UI::Shell::CREDENTIAL_PROVIDER_STATUS_ICON,
    ) -> windows_core::Result<()> {
        Ok(())
    }
}
