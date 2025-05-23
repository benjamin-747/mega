#[cfg(feature = "Storage_Streams")]
pub trait IMidiMessage_Impl: Sized {
    fn Timestamp(&self) -> windows_core::Result<super::super::Foundation::TimeSpan>;
    fn RawData(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer>;
    fn Type(&self) -> windows_core::Result<MidiMessageType>;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IMidiMessage {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiMessage";
}
#[cfg(feature = "Storage_Streams")]
impl IMidiMessage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMidiMessage_Impl, const OFFSET: isize>() -> IMidiMessage_Vtbl {
        unsafe extern "system" fn Timestamp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMidiMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMidiMessage_Impl::Timestamp(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMidiMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMidiMessage_Impl::RawData(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMidiMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MidiMessageType) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMidiMessage_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMidiMessage, OFFSET>(),
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            RawData: RawData::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMidiMessage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Storage_Streams")]
pub trait IMidiOutPort_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn SendMessage(&self, midimessage: Option<&IMidiMessage>) -> windows_core::Result<()>;
    fn SendBuffer(&self, mididata: Option<&super::super::Storage::Streams::IBuffer>) -> windows_core::Result<()>;
    fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IMidiOutPort {
    const NAME: &'static str = "Windows.Devices.Midi.IMidiOutPort";
}
#[cfg(feature = "Storage_Streams")]
impl IMidiOutPort_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMidiOutPort_Impl, const OFFSET: isize>() -> IMidiOutPort_Vtbl {
        unsafe extern "system" fn SendMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMidiOutPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, midimessage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMidiOutPort_Impl::SendMessage(this, windows_core::from_raw_borrowed(&midimessage)).into()
        }
        unsafe extern "system" fn SendBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMidiOutPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mididata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMidiOutPort_Impl::SendBuffer(this, windows_core::from_raw_borrowed(&mididata)).into()
        }
        unsafe extern "system" fn DeviceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMidiOutPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMidiOutPort_Impl::DeviceId(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMidiOutPort, OFFSET>(),
            SendMessage: SendMessage::<Identity, Impl, OFFSET>,
            SendBuffer: SendBuffer::<Identity, Impl, OFFSET>,
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMidiOutPort as windows_core::Interface>::IID
    }
}
