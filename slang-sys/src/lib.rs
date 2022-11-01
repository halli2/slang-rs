#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::{marker::PhantomData, ptr};

    use super::*;

    struct GlobalSession {
        inner: *mut slang_IGlobalSession,
    }

    impl Drop for GlobalSession {
        fn drop(&mut self) {
            unsafe {
                spDestroySession(self.inner);
            }
        }
    }

    #[test]
    fn slang_session() {
        unsafe {
            let slang_bool: SlangBool = true;
            assert!(slang_bool);

            let mut global_session = std::mem::zeroed();
            let result =
                slang_createGlobalSession(SLANG_API_VERSION as SlangInt, &mut global_session);
            assert!(result >= 0);

            let _global_session = GlobalSession {
                inner: global_session,
            };

            // eprintln!("{:?}", global_session);
            // slang_createGlobalSession();
            // TODO:
        }
    }
}
