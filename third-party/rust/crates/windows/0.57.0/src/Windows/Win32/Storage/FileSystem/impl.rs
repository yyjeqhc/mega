#[cfg(feature = "Win32_System_Com")]
pub trait IDiskQuotaControl_Impl: Sized + super::super::System::Com::IConnectionPointContainer_Impl {
    fn Initialize(&self, pszpath: &windows_core::PCWSTR, breadwrite: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetQuotaState(&self, dwstate: u32) -> windows_core::Result<()>;
    fn GetQuotaState(&self, pdwstate: *mut u32) -> windows_core::Result<()>;
    fn SetQuotaLogFlags(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetQuotaLogFlags(&self, pdwflags: *mut u32) -> windows_core::Result<()>;
    fn SetDefaultQuotaThreshold(&self, llthreshold: i64) -> windows_core::Result<()>;
    fn GetDefaultQuotaThreshold(&self, pllthreshold: *mut i64) -> windows_core::Result<()>;
    fn GetDefaultQuotaThresholdText(&self, psztext: &windows_core::PCWSTR, cchtext: u32) -> windows_core::Result<()>;
    fn SetDefaultQuotaLimit(&self, lllimit: i64) -> windows_core::Result<()>;
    fn GetDefaultQuotaLimit(&self, plllimit: *mut i64) -> windows_core::Result<()>;
    fn GetDefaultQuotaLimitText(&self, psztext: &windows_core::PCWSTR, cchtext: u32) -> windows_core::Result<()>;
    fn AddUserSid(&self, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> windows_core::Result<IDiskQuotaUser>;
    fn AddUserName(&self, pszlogonname: &windows_core::PCWSTR, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> windows_core::Result<IDiskQuotaUser>;
    fn DeleteUser(&self, puser: Option<&IDiskQuotaUser>) -> windows_core::Result<()>;
    fn FindUserSid(&self, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE) -> windows_core::Result<IDiskQuotaUser>;
    fn FindUserName(&self, pszlogonname: &windows_core::PCWSTR) -> windows_core::Result<IDiskQuotaUser>;
    fn CreateEnumUsers(&self, rgpusersids: *mut super::super::Foundation::PSID, cpsids: u32, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppenum: *mut Option<IEnumDiskQuotaUsers>) -> windows_core::Result<()>;
    fn CreateUserBatch(&self) -> windows_core::Result<IDiskQuotaUserBatch>;
    fn InvalidateSidNameCache(&self) -> windows_core::Result<()>;
    fn GiveUserNameResolutionPriority(&self, puser: Option<&IDiskQuotaUser>) -> windows_core::Result<()>;
    fn ShutdownNameResolution(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDiskQuotaControl {}
#[cfg(feature = "Win32_System_Com")]
impl IDiskQuotaControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>() -> IDiskQuotaControl_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, breadwrite: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::Initialize(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&breadwrite)).into()
        }
        unsafe extern "system" fn SetQuotaState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstate: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::SetQuotaState(this, core::mem::transmute_copy(&dwstate)).into()
        }
        unsafe extern "system" fn GetQuotaState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::GetQuotaState(this, core::mem::transmute_copy(&pdwstate)).into()
        }
        unsafe extern "system" fn SetQuotaLogFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::SetQuotaLogFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetQuotaLogFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::GetQuotaLogFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetDefaultQuotaThreshold<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llthreshold: i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::SetDefaultQuotaThreshold(this, core::mem::transmute_copy(&llthreshold)).into()
        }
        unsafe extern "system" fn GetDefaultQuotaThreshold<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllthreshold: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::GetDefaultQuotaThreshold(this, core::mem::transmute_copy(&pllthreshold)).into()
        }
        unsafe extern "system" fn GetDefaultQuotaThresholdText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztext: windows_core::PCWSTR, cchtext: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::GetDefaultQuotaThresholdText(this, core::mem::transmute(&psztext), core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn SetDefaultQuotaLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lllimit: i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::SetDefaultQuotaLimit(this, core::mem::transmute_copy(&lllimit)).into()
        }
        unsafe extern "system" fn GetDefaultQuotaLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plllimit: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::GetDefaultQuotaLimit(this, core::mem::transmute_copy(&plllimit)).into()
        }
        unsafe extern "system" fn GetDefaultQuotaLimitText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztext: windows_core::PCWSTR, cchtext: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::GetDefaultQuotaLimitText(this, core::mem::transmute(&psztext), core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn AddUserSid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDiskQuotaControl_Impl::AddUserSid(this, core::mem::transmute_copy(&pusersid), core::mem::transmute_copy(&fnameresolution)) {
                Ok(ok__) => {
                    core::ptr::write(ppuser, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddUserName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlogonname: windows_core::PCWSTR, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDiskQuotaControl_Impl::AddUserName(this, core::mem::transmute(&pszlogonname), core::mem::transmute_copy(&fnameresolution)) {
                Ok(ok__) => {
                    core::ptr::write(ppuser, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::DeleteUser(this, windows_core::from_raw_borrowed(&puser)).into()
        }
        unsafe extern "system" fn FindUserSid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusersid: super::super::Foundation::PSID, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppuser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDiskQuotaControl_Impl::FindUserSid(this, core::mem::transmute_copy(&pusersid), core::mem::transmute_copy(&fnameresolution)) {
                Ok(ok__) => {
                    core::ptr::write(ppuser, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindUserName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlogonname: windows_core::PCWSTR, ppuser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDiskQuotaControl_Impl::FindUserName(this, core::mem::transmute(&pszlogonname)) {
                Ok(ok__) => {
                    core::ptr::write(ppuser, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEnumUsers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgpusersids: *mut super::super::Foundation::PSID, cpsids: u32, fnameresolution: DISKQUOTA_USERNAME_RESOLVE, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::CreateEnumUsers(this, core::mem::transmute_copy(&rgpusersids), core::mem::transmute_copy(&cpsids), core::mem::transmute_copy(&fnameresolution), core::mem::transmute_copy(&ppenum)).into()
        }
        unsafe extern "system" fn CreateUserBatch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDiskQuotaControl_Impl::CreateUserBatch(this) {
                Ok(ok__) => {
                    core::ptr::write(ppbatch, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidateSidNameCache<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::InvalidateSidNameCache(this).into()
        }
        unsafe extern "system" fn GiveUserNameResolutionPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::GiveUserNameResolutionPriority(this, windows_core::from_raw_borrowed(&puser)).into()
        }
        unsafe extern "system" fn ShutdownNameResolution<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaControl_Impl::ShutdownNameResolution(this).into()
        }
        Self {
            base__: super::super::System::Com::IConnectionPointContainer_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetQuotaState: SetQuotaState::<Identity, Impl, OFFSET>,
            GetQuotaState: GetQuotaState::<Identity, Impl, OFFSET>,
            SetQuotaLogFlags: SetQuotaLogFlags::<Identity, Impl, OFFSET>,
            GetQuotaLogFlags: GetQuotaLogFlags::<Identity, Impl, OFFSET>,
            SetDefaultQuotaThreshold: SetDefaultQuotaThreshold::<Identity, Impl, OFFSET>,
            GetDefaultQuotaThreshold: GetDefaultQuotaThreshold::<Identity, Impl, OFFSET>,
            GetDefaultQuotaThresholdText: GetDefaultQuotaThresholdText::<Identity, Impl, OFFSET>,
            SetDefaultQuotaLimit: SetDefaultQuotaLimit::<Identity, Impl, OFFSET>,
            GetDefaultQuotaLimit: GetDefaultQuotaLimit::<Identity, Impl, OFFSET>,
            GetDefaultQuotaLimitText: GetDefaultQuotaLimitText::<Identity, Impl, OFFSET>,
            AddUserSid: AddUserSid::<Identity, Impl, OFFSET>,
            AddUserName: AddUserName::<Identity, Impl, OFFSET>,
            DeleteUser: DeleteUser::<Identity, Impl, OFFSET>,
            FindUserSid: FindUserSid::<Identity, Impl, OFFSET>,
            FindUserName: FindUserName::<Identity, Impl, OFFSET>,
            CreateEnumUsers: CreateEnumUsers::<Identity, Impl, OFFSET>,
            CreateUserBatch: CreateUserBatch::<Identity, Impl, OFFSET>,
            InvalidateSidNameCache: InvalidateSidNameCache::<Identity, Impl, OFFSET>,
            GiveUserNameResolutionPriority: GiveUserNameResolutionPriority::<Identity, Impl, OFFSET>,
            ShutdownNameResolution: ShutdownNameResolution::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiskQuotaControl as windows_core::Interface>::IID || iid == &<super::super::System::Com::IConnectionPointContainer as windows_core::Interface>::IID
    }
}
pub trait IDiskQuotaEvents_Impl: Sized {
    fn OnUserNameChanged(&self, puser: Option<&IDiskQuotaUser>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDiskQuotaEvents {}
impl IDiskQuotaEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaEvents_Impl, const OFFSET: isize>() -> IDiskQuotaEvents_Vtbl {
        unsafe extern "system" fn OnUserNameChanged<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaEvents_Impl::OnUserNameChanged(this, windows_core::from_raw_borrowed(&puser)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUserNameChanged: OnUserNameChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiskQuotaEvents as windows_core::Interface>::IID
    }
}
pub trait IDiskQuotaUser_Impl: Sized {
    fn GetID(&self, pulid: *mut u32) -> windows_core::Result<()>;
    fn GetName(&self, pszaccountcontainer: &windows_core::PCWSTR, cchaccountcontainer: u32, pszlogonname: &windows_core::PCWSTR, cchlogonname: u32, pszdisplayname: &windows_core::PCWSTR, cchdisplayname: u32) -> windows_core::Result<()>;
    fn GetSidLength(&self, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn GetSid(&self, pbsidbuffer: *mut u8, cbsidbuffer: u32) -> windows_core::Result<()>;
    fn GetQuotaThreshold(&self, pllthreshold: *mut i64) -> windows_core::Result<()>;
    fn GetQuotaThresholdText(&self, psztext: &windows_core::PCWSTR, cchtext: u32) -> windows_core::Result<()>;
    fn GetQuotaLimit(&self, plllimit: *mut i64) -> windows_core::Result<()>;
    fn GetQuotaLimitText(&self, psztext: &windows_core::PCWSTR, cchtext: u32) -> windows_core::Result<()>;
    fn GetQuotaUsed(&self, pllused: *mut i64) -> windows_core::Result<()>;
    fn GetQuotaUsedText(&self, psztext: &windows_core::PCWSTR, cchtext: u32) -> windows_core::Result<()>;
    fn GetQuotaInformation(&self, pbquotainfo: *mut core::ffi::c_void, cbquotainfo: u32) -> windows_core::Result<()>;
    fn SetQuotaThreshold(&self, llthreshold: i64, fwritethrough: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetQuotaLimit(&self, lllimit: i64, fwritethrough: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Invalidate(&self) -> windows_core::Result<()>;
    fn GetAccountStatus(&self, pdwstatus: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDiskQuotaUser {}
impl IDiskQuotaUser_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>() -> IDiskQuotaUser_Vtbl {
        unsafe extern "system" fn GetID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulid: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::GetID(this, core::mem::transmute_copy(&pulid)).into()
        }
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszaccountcontainer: windows_core::PCWSTR, cchaccountcontainer: u32, pszlogonname: windows_core::PCWSTR, cchlogonname: u32, pszdisplayname: windows_core::PCWSTR, cchdisplayname: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::GetName(this, core::mem::transmute(&pszaccountcontainer), core::mem::transmute_copy(&cchaccountcontainer), core::mem::transmute(&pszlogonname), core::mem::transmute_copy(&cchlogonname), core::mem::transmute(&pszdisplayname), core::mem::transmute_copy(&cchdisplayname)).into()
        }
        unsafe extern "system" fn GetSidLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwlength: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::GetSidLength(this, core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetSid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsidbuffer: *mut u8, cbsidbuffer: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::GetSid(this, core::mem::transmute_copy(&pbsidbuffer), core::mem::transmute_copy(&cbsidbuffer)).into()
        }
        unsafe extern "system" fn GetQuotaThreshold<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllthreshold: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::GetQuotaThreshold(this, core::mem::transmute_copy(&pllthreshold)).into()
        }
        unsafe extern "system" fn GetQuotaThresholdText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztext: windows_core::PCWSTR, cchtext: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::GetQuotaThresholdText(this, core::mem::transmute(&psztext), core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetQuotaLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plllimit: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::GetQuotaLimit(this, core::mem::transmute_copy(&plllimit)).into()
        }
        unsafe extern "system" fn GetQuotaLimitText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztext: windows_core::PCWSTR, cchtext: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::GetQuotaLimitText(this, core::mem::transmute(&psztext), core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetQuotaUsed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllused: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::GetQuotaUsed(this, core::mem::transmute_copy(&pllused)).into()
        }
        unsafe extern "system" fn GetQuotaUsedText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztext: windows_core::PCWSTR, cchtext: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::GetQuotaUsedText(this, core::mem::transmute(&psztext), core::mem::transmute_copy(&cchtext)).into()
        }
        unsafe extern "system" fn GetQuotaInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbquotainfo: *mut core::ffi::c_void, cbquotainfo: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::GetQuotaInformation(this, core::mem::transmute_copy(&pbquotainfo), core::mem::transmute_copy(&cbquotainfo)).into()
        }
        unsafe extern "system" fn SetQuotaThreshold<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llthreshold: i64, fwritethrough: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::SetQuotaThreshold(this, core::mem::transmute_copy(&llthreshold), core::mem::transmute_copy(&fwritethrough)).into()
        }
        unsafe extern "system" fn SetQuotaLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lllimit: i64, fwritethrough: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::SetQuotaLimit(this, core::mem::transmute_copy(&lllimit), core::mem::transmute_copy(&fwritethrough)).into()
        }
        unsafe extern "system" fn Invalidate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::Invalidate(this).into()
        }
        unsafe extern "system" fn GetAccountStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUser_Impl::GetAccountStatus(this, core::mem::transmute_copy(&pdwstatus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetSidLength: GetSidLength::<Identity, Impl, OFFSET>,
            GetSid: GetSid::<Identity, Impl, OFFSET>,
            GetQuotaThreshold: GetQuotaThreshold::<Identity, Impl, OFFSET>,
            GetQuotaThresholdText: GetQuotaThresholdText::<Identity, Impl, OFFSET>,
            GetQuotaLimit: GetQuotaLimit::<Identity, Impl, OFFSET>,
            GetQuotaLimitText: GetQuotaLimitText::<Identity, Impl, OFFSET>,
            GetQuotaUsed: GetQuotaUsed::<Identity, Impl, OFFSET>,
            GetQuotaUsedText: GetQuotaUsedText::<Identity, Impl, OFFSET>,
            GetQuotaInformation: GetQuotaInformation::<Identity, Impl, OFFSET>,
            SetQuotaThreshold: SetQuotaThreshold::<Identity, Impl, OFFSET>,
            SetQuotaLimit: SetQuotaLimit::<Identity, Impl, OFFSET>,
            Invalidate: Invalidate::<Identity, Impl, OFFSET>,
            GetAccountStatus: GetAccountStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiskQuotaUser as windows_core::Interface>::IID
    }
}
pub trait IDiskQuotaUserBatch_Impl: Sized {
    fn Add(&self, puser: Option<&IDiskQuotaUser>) -> windows_core::Result<()>;
    fn Remove(&self, puser: Option<&IDiskQuotaUser>) -> windows_core::Result<()>;
    fn RemoveAll(&self) -> windows_core::Result<()>;
    fn FlushToDisk(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDiskQuotaUserBatch {}
impl IDiskQuotaUserBatch_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: isize>() -> IDiskQuotaUserBatch_Vtbl {
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUserBatch_Impl::Add(this, windows_core::from_raw_borrowed(&puser)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUserBatch_Impl::Remove(this, windows_core::from_raw_borrowed(&puser)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUserBatch_Impl::RemoveAll(this).into()
        }
        unsafe extern "system" fn FlushToDisk<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiskQuotaUserBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDiskQuotaUserBatch_Impl::FlushToDisk(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
            FlushToDisk: FlushToDisk::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiskQuotaUserBatch as windows_core::Interface>::IID
    }
}
pub trait IEnumDiskQuotaUsers_Impl: Sized {
    fn Next(&self, cusers: u32, rgusers: *mut Option<IDiskQuotaUser>, pcusersfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cusers: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDiskQuotaUsers>;
}
impl windows_core::RuntimeName for IEnumDiskQuotaUsers {}
impl IEnumDiskQuotaUsers_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: isize>() -> IEnumDiskQuotaUsers_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cusers: u32, rgusers: *mut *mut core::ffi::c_void, pcusersfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDiskQuotaUsers_Impl::Next(this, core::mem::transmute_copy(&cusers), core::mem::transmute_copy(&rgusers), core::mem::transmute_copy(&pcusersfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cusers: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDiskQuotaUsers_Impl::Skip(this, core::mem::transmute_copy(&cusers)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumDiskQuotaUsers_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiskQuotaUsers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumDiskQuotaUsers_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDiskQuotaUsers as windows_core::Interface>::IID
    }
}
