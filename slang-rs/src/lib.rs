use slang_sys::root::*;

pub struct GlobalSession {
    inner: *mut slang::IGlobalSession,
}

impl Drop for GlobalSession {
    fn drop(&mut self) {
        unsafe {
            spDestroySession(self.inner);
        }
    }
}

impl GlobalSession {
    pub fn new() -> Self {
        unsafe {
            let mut global_session = std::mem::zeroed();
            let result =
                slang_createGlobalSession(SLANG_API_VERSION as SlangInt, &mut global_session);

            let global_session = global_session.as_mut().unwrap();
            let mut obj = global_session._base;
            let vtable = obj.vtable_.as_ref().unwrap();
            let ptr: *mut ISlangUnknown = &mut obj;
            let res = (vtable.ISlangUnknown_release)(ptr);
            println!("{:?}", res);
            Self {
                inner: global_session,
            }
        }
    }
    pub fn get(&self) -> &slang::IGlobalSession {
        unsafe { self.inner.as_ref().unwrap() }
    }

    fn create_session(&self) {
        unsafe {}
    }
}

impl Default for GlobalSession {
    fn default() -> Self {
        Self::new()
    }
}
