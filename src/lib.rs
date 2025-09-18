use std::{ffi, mem, ptr};

use windows::Win32::Foundation::{
    CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL,
    E_POINTER, S_OK,
};
use windows::Win32::System::Com::{IClassFactory, IClassFactory_Impl};
use windows::Win32::UI::Shell::ICredentialProvider;
use windows_core::{BOOL, GUID, HRESULT, IUnknown, Interface, implement};

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

        // We're only handling requests for `IID_ICredentialProvider`
        if riid != ICredentialProvider::IID {
            return Err(E_NOINTERFACE.into());
        }

        // Construct credential provider and return it as an `ICredentialProvider`
        // interface
        let provider: ICredentialProvider = CredentialProvider::CredentialProvider {}.into();
        unsafe { *ppvobject = mem::transmute(provider) };
        Ok(())
    }

    fn LockServer(&self, flock: BOOL) -> windows::core::Result<()> {
        let _ = flock;
        Err(E_NOTIMPL.into())
    }
}
