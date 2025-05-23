pub trait ICallFrame_Impl: Sized {
    fn GetInfo(&self, pinfo: *mut CALLFRAMEINFO) -> windows_core::Result<()>;
    fn GetIIDAndMethod(&self, piid: *mut windows_core::GUID, pimethod: *mut u32) -> windows_core::Result<()>;
    fn GetNames(&self, pwszinterface: *mut windows_core::PWSTR, pwszmethod: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetStackLocation(&self) -> *mut core::ffi::c_void;
    fn SetStackLocation(&self, pvstack: *const core::ffi::c_void);
    fn SetReturnValue(&self, hr: windows_core::HRESULT);
    fn GetReturnValue(&self) -> windows_core::Result<()>;
    fn GetParamInfo(&self, iparam: u32) -> windows_core::Result<CALLFRAMEPARAMINFO>;
    fn SetParam(&self, iparam: u32, pvar: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetParam(&self, iparam: u32) -> windows_core::Result<windows_core::VARIANT>;
    fn Copy(&self, copycontrol: CALLFRAME_COPY, pwalker: Option<&ICallFrameWalker>) -> windows_core::Result<ICallFrame>;
    fn Free(&self, pframeargsdest: Option<&ICallFrame>, pwalkerdestfree: Option<&ICallFrameWalker>, pwalkercopy: Option<&ICallFrameWalker>, freeflags: u32, pwalkerfree: Option<&ICallFrameWalker>, nullflags: u32) -> windows_core::Result<()>;
    fn FreeParam(&self, iparam: u32, freeflags: u32, pwalkerfree: Option<&ICallFrameWalker>, nullflags: u32) -> windows_core::Result<()>;
    fn WalkFrame(&self, walkwhat: u32, pwalker: Option<&ICallFrameWalker>) -> windows_core::Result<()>;
    fn GetMarshalSizeMax(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS) -> windows_core::Result<u32>;
    fn Marshal(&self, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> windows_core::Result<()>;
    fn Unmarshal(&self, pbuffer: *const core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::Result<u32>;
    fn ReleaseMarshalData(&self, pbuffer: *const core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::Result<()>;
    fn Invoke(&self, pvreceiver: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICallFrame {}
impl ICallFrame_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICallFrame_Vtbl
    where
        Identity: ICallFrame_Impl,
    {
        unsafe extern "system" fn GetInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut CALLFRAMEINFO) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::GetInfo(this, core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn GetIIDAndMethod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID, pimethod: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::GetIIDAndMethod(this, core::mem::transmute_copy(&piid), core::mem::transmute_copy(&pimethod)).into()
        }
        unsafe extern "system" fn GetNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszinterface: *mut windows_core::PWSTR, pwszmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::GetNames(this, core::mem::transmute_copy(&pwszinterface), core::mem::transmute_copy(&pwszmethod)).into()
        }
        unsafe extern "system" fn GetStackLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut core::ffi::c_void
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::GetStackLocation(this)
        }
        unsafe extern "system" fn SetStackLocation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvstack: *const core::ffi::c_void)
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::SetStackLocation(this, core::mem::transmute_copy(&pvstack))
        }
        unsafe extern "system" fn SetReturnValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT)
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::SetReturnValue(this, core::mem::transmute_copy(&hr))
        }
        unsafe extern "system" fn GetReturnValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::GetReturnValue(this).into()
        }
        unsafe extern "system" fn GetParamInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iparam: u32, pinfo: *mut CALLFRAMEPARAMINFO) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICallFrame_Impl::GetParamInfo(this, core::mem::transmute_copy(&iparam)) {
                Ok(ok__) => {
                    pinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParam<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iparam: u32, pvar: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::SetParam(this, core::mem::transmute_copy(&iparam), core::mem::transmute_copy(&pvar)).into()
        }
        unsafe extern "system" fn GetParam<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iparam: u32, pvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICallFrame_Impl::GetParam(this, core::mem::transmute_copy(&iparam)) {
                Ok(ok__) => {
                    pvar.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, copycontrol: CALLFRAME_COPY, pwalker: *mut core::ffi::c_void, ppframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICallFrame_Impl::Copy(this, core::mem::transmute_copy(&copycontrol), windows_core::from_raw_borrowed(&pwalker)) {
                Ok(ok__) => {
                    ppframe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Free<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pframeargsdest: *mut core::ffi::c_void, pwalkerdestfree: *mut core::ffi::c_void, pwalkercopy: *mut core::ffi::c_void, freeflags: u32, pwalkerfree: *mut core::ffi::c_void, nullflags: u32) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::Free(this, windows_core::from_raw_borrowed(&pframeargsdest), windows_core::from_raw_borrowed(&pwalkerdestfree), windows_core::from_raw_borrowed(&pwalkercopy), core::mem::transmute_copy(&freeflags), windows_core::from_raw_borrowed(&pwalkerfree), core::mem::transmute_copy(&nullflags)).into()
        }
        unsafe extern "system" fn FreeParam<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iparam: u32, freeflags: u32, pwalkerfree: *mut core::ffi::c_void, nullflags: u32) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::FreeParam(this, core::mem::transmute_copy(&iparam), core::mem::transmute_copy(&freeflags), windows_core::from_raw_borrowed(&pwalkerfree), core::mem::transmute_copy(&nullflags)).into()
        }
        unsafe extern "system" fn WalkFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, walkwhat: u32, pwalker: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::WalkFrame(this, core::mem::transmute_copy(&walkwhat), windows_core::from_raw_borrowed(&pwalker)).into()
        }
        unsafe extern "system" fn GetMarshalSizeMax<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pcbbufferneeded: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICallFrame_Impl::GetMarshalSizeMax(this, core::mem::transmute_copy(&pmshlcontext), core::mem::transmute_copy(&mshlflags)) {
                Ok(ok__) => {
                    pcbbufferneeded.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Marshal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmshlcontext: *const CALLFRAME_MARSHALCONTEXT, mshlflags: super::MSHLFLAGS, pbuffer: *const core::ffi::c_void, cbbuffer: u32, pcbbufferused: *mut u32, pdatarep: *mut u32, prpcflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::Marshal(this, core::mem::transmute_copy(&pmshlcontext), core::mem::transmute_copy(&mshlflags), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&pcbbufferused), core::mem::transmute_copy(&pdatarep), core::mem::transmute_copy(&prpcflags)).into()
        }
        unsafe extern "system" fn Unmarshal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *const core::ffi::c_void, cbbuffer: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICallFrame_Impl::Unmarshal(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&datarep), core::mem::transmute_copy(&pcontext)) {
                Ok(ok__) => {
                    pcbunmarshalled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseMarshalData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *const core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::ReleaseMarshalData(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ibfirstrelease), core::mem::transmute_copy(&datarep), core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn Invoke<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvreceiver: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICallFrame_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrame_Impl::Invoke(this, core::mem::transmute_copy(&pvreceiver)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, OFFSET>,
            GetIIDAndMethod: GetIIDAndMethod::<Identity, OFFSET>,
            GetNames: GetNames::<Identity, OFFSET>,
            GetStackLocation: GetStackLocation::<Identity, OFFSET>,
            SetStackLocation: SetStackLocation::<Identity, OFFSET>,
            SetReturnValue: SetReturnValue::<Identity, OFFSET>,
            GetReturnValue: GetReturnValue::<Identity, OFFSET>,
            GetParamInfo: GetParamInfo::<Identity, OFFSET>,
            SetParam: SetParam::<Identity, OFFSET>,
            GetParam: GetParam::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Free: Free::<Identity, OFFSET>,
            FreeParam: FreeParam::<Identity, OFFSET>,
            WalkFrame: WalkFrame::<Identity, OFFSET>,
            GetMarshalSizeMax: GetMarshalSizeMax::<Identity, OFFSET>,
            Marshal: Marshal::<Identity, OFFSET>,
            Unmarshal: Unmarshal::<Identity, OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallFrame as windows_core::Interface>::IID
    }
}
pub trait ICallFrameEvents_Impl: Sized {
    fn OnCall(&self, pframe: Option<&ICallFrame>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICallFrameEvents {}
impl ICallFrameEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICallFrameEvents_Vtbl
    where
        Identity: ICallFrameEvents_Impl,
    {
        unsafe extern "system" fn OnCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pframe: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICallFrameEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrameEvents_Impl::OnCall(this, windows_core::from_raw_borrowed(&pframe)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCall: OnCall::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallFrameEvents as windows_core::Interface>::IID
    }
}
pub trait ICallFrameWalker_Impl: Sized {
    fn OnWalkInterface(&self, iid: *const windows_core::GUID, ppvinterface: *const *const core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICallFrameWalker {}
impl ICallFrameWalker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICallFrameWalker_Vtbl
    where
        Identity: ICallFrameWalker_Impl,
    {
        unsafe extern "system" fn OnWalkInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, ppvinterface: *const *const core::ffi::c_void, fin: super::super::super::Foundation::BOOL, fout: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ICallFrameWalker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallFrameWalker_Impl::OnWalkInterface(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvinterface), core::mem::transmute_copy(&fin), core::mem::transmute_copy(&fout)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnWalkInterface: OnWalkInterface::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallFrameWalker as windows_core::Interface>::IID
    }
}
pub trait ICallIndirect_Impl: Sized {
    fn CallIndirect(&self, phrreturn: *mut windows_core::HRESULT, imethod: u32, pvargs: *const core::ffi::c_void, cbargs: *mut u32) -> windows_core::Result<()>;
    fn GetMethodInfo(&self, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetStackSize(&self, imethod: u32) -> windows_core::Result<u32>;
    fn GetIID(&self, piid: *mut windows_core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICallIndirect {}
impl ICallIndirect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICallIndirect_Vtbl
    where
        Identity: ICallIndirect_Impl,
    {
        unsafe extern "system" fn CallIndirect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrreturn: *mut windows_core::HRESULT, imethod: u32, pvargs: *const core::ffi::c_void, cbargs: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICallIndirect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallIndirect_Impl::CallIndirect(this, core::mem::transmute_copy(&phrreturn), core::mem::transmute_copy(&imethod), core::mem::transmute_copy(&pvargs), core::mem::transmute_copy(&cbargs)).into()
        }
        unsafe extern "system" fn GetMethodInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imethod: u32, pinfo: *mut CALLFRAMEINFO, pwszmethod: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICallIndirect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallIndirect_Impl::GetMethodInfo(this, core::mem::transmute_copy(&imethod), core::mem::transmute_copy(&pinfo), core::mem::transmute_copy(&pwszmethod)).into()
        }
        unsafe extern "system" fn GetStackSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imethod: u32, cbargs: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICallIndirect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICallIndirect_Impl::GetStackSize(this, core::mem::transmute_copy(&imethod)) {
                Ok(ok__) => {
                    cbargs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID, pfderivesfromidispatch: *mut super::super::super::Foundation::BOOL, pcmethod: *mut u32, pwszinterface: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICallIndirect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallIndirect_Impl::GetIID(this, core::mem::transmute_copy(&piid), core::mem::transmute_copy(&pfderivesfromidispatch), core::mem::transmute_copy(&pcmethod), core::mem::transmute_copy(&pwszinterface)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CallIndirect: CallIndirect::<Identity, OFFSET>,
            GetMethodInfo: GetMethodInfo::<Identity, OFFSET>,
            GetStackSize: GetStackSize::<Identity, OFFSET>,
            GetIID: GetIID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallIndirect as windows_core::Interface>::IID
    }
}
pub trait ICallInterceptor_Impl: Sized + ICallIndirect_Impl {
    fn RegisterSink(&self, psink: Option<&ICallFrameEvents>) -> windows_core::Result<()>;
    fn GetRegisteredSink(&self) -> windows_core::Result<ICallFrameEvents>;
}
impl windows_core::RuntimeName for ICallInterceptor {}
impl ICallInterceptor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICallInterceptor_Vtbl
    where
        Identity: ICallInterceptor_Impl,
    {
        unsafe extern "system" fn RegisterSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICallInterceptor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallInterceptor_Impl::RegisterSink(this, windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn GetRegisteredSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICallInterceptor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICallInterceptor_Impl::GetRegisteredSink(this) {
                Ok(ok__) => {
                    ppsink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICallIndirect_Vtbl::new::<Identity, OFFSET>(),
            RegisterSink: RegisterSink::<Identity, OFFSET>,
            GetRegisteredSink: GetRegisteredSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallInterceptor as windows_core::Interface>::IID || iid == &<ICallIndirect as windows_core::Interface>::IID
    }
}
pub trait ICallUnmarshal_Impl: Sized {
    fn Unmarshal(&self, imethod: u32, pbuffer: *const core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut Option<ICallFrame>) -> windows_core::Result<()>;
    fn ReleaseMarshalData(&self, imethod: u32, pbuffer: *const core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICallUnmarshal {}
impl ICallUnmarshal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICallUnmarshal_Vtbl
    where
        Identity: ICallUnmarshal_Impl,
    {
        unsafe extern "system" fn Unmarshal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imethod: u32, pbuffer: *const core::ffi::c_void, cbbuffer: u32, fforcebuffercopy: super::super::super::Foundation::BOOL, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT, pcbunmarshalled: *mut u32, ppframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICallUnmarshal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallUnmarshal_Impl::Unmarshal(this, core::mem::transmute_copy(&imethod), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&fforcebuffercopy), core::mem::transmute_copy(&datarep), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pcbunmarshalled), core::mem::transmute_copy(&ppframe)).into()
        }
        unsafe extern "system" fn ReleaseMarshalData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imethod: u32, pbuffer: *const core::ffi::c_void, cbbuffer: u32, ibfirstrelease: u32, datarep: u32, pcontext: *const CALLFRAME_MARSHALCONTEXT) -> windows_core::HRESULT
        where
            Identity: ICallUnmarshal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICallUnmarshal_Impl::ReleaseMarshalData(this, core::mem::transmute_copy(&imethod), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&ibfirstrelease), core::mem::transmute_copy(&datarep), core::mem::transmute_copy(&pcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Unmarshal: Unmarshal::<Identity, OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallUnmarshal as windows_core::Interface>::IID
    }
}
pub trait IInterfaceRelated_Impl: Sized {
    fn SetIID(&self, iid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetIID(&self) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IInterfaceRelated {}
impl IInterfaceRelated_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInterfaceRelated_Vtbl
    where
        Identity: IInterfaceRelated_Impl,
    {
        unsafe extern "system" fn SetIID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IInterfaceRelated_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInterfaceRelated_Impl::SetIID(this, core::mem::transmute_copy(&iid)).into()
        }
        unsafe extern "system" fn GetIID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IInterfaceRelated_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInterfaceRelated_Impl::GetIID(this) {
                Ok(ok__) => {
                    piid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetIID: SetIID::<Identity, OFFSET>, GetIID: GetIID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInterfaceRelated as windows_core::Interface>::IID
    }
}
