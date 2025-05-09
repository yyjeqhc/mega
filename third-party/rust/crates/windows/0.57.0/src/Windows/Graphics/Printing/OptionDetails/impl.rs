pub trait IPrintCustomOptionDetails_Impl: Sized + IPrintOptionDetails_Impl {
    fn SetDisplayName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IPrintCustomOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails";
}
impl IPrintCustomOptionDetails_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCustomOptionDetails_Impl, const OFFSET: isize>() -> IPrintCustomOptionDetails_Vtbl {
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCustomOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintCustomOptionDetails_Impl::SetDisplayName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintCustomOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintCustomOptionDetails_Impl::DisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintCustomOptionDetails, OFFSET>(),
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintCustomOptionDetails as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPrintItemListOptionDetails_Impl: Sized + IPrintOptionDetails_Impl {
    fn Items(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<windows_core::IInspectable>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IPrintItemListOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails";
}
#[cfg(feature = "Foundation_Collections")]
impl IPrintItemListOptionDetails_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintItemListOptionDetails_Impl, const OFFSET: isize>() -> IPrintItemListOptionDetails_Vtbl {
        unsafe extern "system" fn Items<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintItemListOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintItemListOptionDetails_Impl::Items(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintItemListOptionDetails, OFFSET>(), Items: Items::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintItemListOptionDetails as windows_core::Interface>::IID
    }
}
pub trait IPrintNumberOptionDetails_Impl: Sized + IPrintOptionDetails_Impl {
    fn MinValue(&self) -> windows_core::Result<u32>;
    fn MaxValue(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IPrintNumberOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintNumberOptionDetails";
}
impl IPrintNumberOptionDetails_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintNumberOptionDetails_Impl, const OFFSET: isize>() -> IPrintNumberOptionDetails_Vtbl {
        unsafe extern "system" fn MinValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintNumberOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintNumberOptionDetails_Impl::MinValue(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintNumberOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintNumberOptionDetails_Impl::MaxValue(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintNumberOptionDetails, OFFSET>(),
            MinValue: MinValue::<Identity, Impl, OFFSET>,
            MaxValue: MaxValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintNumberOptionDetails as windows_core::Interface>::IID
    }
}
pub trait IPrintOptionDetails_Impl: Sized {
    fn OptionId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn OptionType(&self) -> windows_core::Result<PrintOptionType>;
    fn SetErrorText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn ErrorText(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetState(&self, value: PrintOptionStates) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<PrintOptionStates>;
    fn Value(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn TrySetValue(&self, value: Option<&windows_core::IInspectable>) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for IPrintOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails";
}
impl IPrintOptionDetails_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>() -> IPrintOptionDetails_Vtbl {
        unsafe extern "system" fn OptionId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintOptionDetails_Impl::OptionId(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintOptionType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintOptionDetails_Impl::OptionType(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintOptionDetails_Impl::SetErrorText(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ErrorText<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintOptionDetails_Impl::ErrorText(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PrintOptionStates) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPrintOptionDetails_Impl::SetState(this, value).into()
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PrintOptionStates) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintOptionDetails_Impl::State(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintOptionDetails_Impl::Value(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintOptionDetails_Impl::TrySetValue(this, windows_core::from_raw_borrowed(&value)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintOptionDetails, OFFSET>(),
            OptionId: OptionId::<Identity, Impl, OFFSET>,
            OptionType: OptionType::<Identity, Impl, OFFSET>,
            SetErrorText: SetErrorText::<Identity, Impl, OFFSET>,
            ErrorText: ErrorText::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            TrySetValue: TrySetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintOptionDetails as windows_core::Interface>::IID
    }
}
pub trait IPrintTextOptionDetails_Impl: Sized + IPrintOptionDetails_Impl {
    fn MaxCharacters(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IPrintTextOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.IPrintTextOptionDetails";
}
impl IPrintTextOptionDetails_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTextOptionDetails_Impl, const OFFSET: isize>() -> IPrintTextOptionDetails_Vtbl {
        unsafe extern "system" fn MaxCharacters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPrintTextOptionDetails_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPrintTextOptionDetails_Impl::MaxCharacters(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintTextOptionDetails, OFFSET>(),
            MaxCharacters: MaxCharacters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTextOptionDetails as windows_core::Interface>::IID
    }
}
