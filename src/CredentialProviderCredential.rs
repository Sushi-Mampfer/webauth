use std::{ffi::OsString, os::windows::ffi::OsStrExt};

use windows::Win32::{
    Foundation::E_NOTIMPL,
    Graphics::Gdi::{
        BLACKNESS, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, HBITMAP, PatBlt,
        SelectObject,
    },
    Security::Credentials::{CRED_PACK_PROTECTED_CREDENTIALS, CredPackAuthenticationBufferW},
    System::Com::CoTaskMemAlloc,
    UI::Shell::{
        CPFIS_NONE, CPFS_DISPLAY_IN_BOTH, CPGSR_NO_CREDENTIAL_FINISHED,
        CPGSR_RETURN_CREDENTIAL_FINISHED, CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION,
        ICredentialProviderCredential, ICredentialProviderCredential_Impl,
    },
};
use windows_core::{GUID, PWSTR, implement};

#[implement(ICredentialProviderCredential)]
pub struct CredentialProviderCredential();

impl ICredentialProviderCredential_Impl for CredentialProviderCredential_Impl {
    fn Advise(
        &self,
        pcpce: windows_core::Ref<windows::Win32::UI::Shell::ICredentialProviderCredentialEvents>,
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
        dwfieldid: u32,
        pcpfs: *mut windows::Win32::UI::Shell::CREDENTIAL_PROVIDER_FIELD_STATE,
        pcpfis: *mut windows::Win32::UI::Shell::CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE,
    ) -> windows_core::Result<()> {
        unsafe {
            match dwfieldid {
                0 => {
                    *pcpfs = CPFS_DISPLAY_IN_BOTH;
                    *pcpfis = CPFIS_NONE;
                    Ok(())
                }
                1 => {
                    *pcpfs = CPFS_DISPLAY_IN_BOTH;
                    *pcpfis = CPFIS_NONE;
                    Ok(())
                }
                2 => {
                    *pcpfs = CPFS_DISPLAY_IN_BOTH;
                    *pcpfis = CPFIS_NONE;
                    Ok(())
                }
                _ => Err(E_NOTIMPL.into()),
            }
        }
    }

    fn GetStringValue(&self, dwfieldid: u32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let wide: Vec<u16> = OsString::from("Test Provider")
                .encode_wide()
                .chain([0])
                .collect();
            let size = wide.len() * size_of::<u16>();
            let label = CoTaskMemAlloc(size) as *mut u16;
            label.copy_from_nonoverlapping(wide.as_ptr(), wide.len());
            Ok(PWSTR(label))
        }
    }

    fn GetBitmapValue(
        &self,
        dwfieldid: u32,
    ) -> windows_core::Result<windows::Win32::Graphics::Gdi::HBITMAP> {
        unsafe {
            let hdc = CreateCompatibleDC(None);
            let map = CreateCompatibleBitmap(hdc, 10, 10);
            let map = SelectObject(hdc, map.into());
            PatBlt(hdc, 0, 0, 10, 10, BLACKNESS);
            let map = SelectObject(hdc, map);
            DeleteDC(hdc);
            Ok(HBITMAP(map.0))
        }
    }

    fn GetCheckboxValue(
        &self,
        dwfieldid: u32,
        pbchecked: *mut windows_core::BOOL,
        ppszlabel: *mut windows_core::PWSTR,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetSubmitButtonValue(&self, dwfieldid: u32) -> windows_core::Result<u32> {
        Ok(1)
    }

    fn GetComboBoxValueCount(
        &self,
        dwfieldid: u32,
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
        dwfieldid: u32,
        dwitem: u32,
    ) -> windows_core::Result<windows_core::PWSTR> {
        Err(E_NOTIMPL.into())
    }

    fn SetStringValue(
        &self,
        dwfieldid: u32,
        psz: &windows_core::PCWSTR,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn SetCheckboxValue(
        &self,
        dwfieldid: u32,
        bchecked: windows_core::BOOL,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn SetComboBoxSelectedValue(
        &self,
        dwfieldid: u32,
        dwselecteditem: u32,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn CommandLinkClicked(&self, dwfieldid: u32) -> windows_core::Result<()> {
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
                user(),
                pasw(),
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
                clsidCredentialProvider: GUID::from_u128(0x7d6836a5_c203_47e2_8fe5_eca9159e7d7e),
                cbSerialization: buffer_size,
                rgbSerialization: buffer,
            });
            *pcpcs = *mem_pcpcs;
        }
        Ok(())
    }

    fn ReportResult(
        &self,
        ntsstatus: windows::Win32::Foundation::NTSTATUS,
        ntssubstatus: windows::Win32::Foundation::NTSTATUS,
        ppszoptionalstatustext: *mut windows_core::PWSTR,
        pcpsioptionalstatusicon: *mut windows::Win32::UI::Shell::CREDENTIAL_PROVIDER_STATUS_ICON,
    ) -> windows_core::Result<()> {
        Ok(())
    }
}

fn user() -> PWSTR {
    unsafe {
        let wide: Vec<u16> = OsString::from("SUSHI-MAMPFER\\test")
            .encode_wide()
            .chain([0])
            .collect();
        let size = wide.len() * size_of::<u16>();
        let label = CoTaskMemAlloc(size) as *mut u16;
        label.copy_from_nonoverlapping(wide.as_ptr(), wide.len());
        PWSTR(label)
    }
}

fn pasw() -> PWSTR {
    unsafe {
        let wide: Vec<u16> = OsString::from("Test").encode_wide().chain([0]).collect();
        let size = wide.len() * size_of::<u16>();
        let label = CoTaskMemAlloc(size) as *mut u16;
        label.copy_from_nonoverlapping(wide.as_ptr(), wide.len());
        PWSTR(label)
    }
}
