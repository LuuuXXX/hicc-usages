use crate::export_class;
use alloc::string::String;

#[export_class(in_hicc)]
impl String {
    fn len(&self) -> usize;
    fn push_str(&mut self, s: &str);
    fn as_str(&self) -> &str;
    fn as_bytes(&self) -> &[u8];
    fn push_cstr(&mut self, s: *const i8) {
        if s.is_null() {
            return;
        }
        unsafe {
            let mut len = 0usize;
            while *s.add(len) != 0 {
                len += 1;
            }
            let bytes = core::slice::from_raw_parts(s as *const u8, len);
            if let Ok(rust_str) = core::str::from_utf8(bytes) {
                self.push_str(rust_str);
            }
        }
    }
    fn insert_str(&mut self, idx: usize, s: &str);
    fn insert_cstr(&mut self, idx: usize, s: *const i8) {
        if s.is_null() {
            return;
        }
        unsafe {
            let mut len = 0usize;
            while *s.add(len) != 0 {
                len += 1;
            }
            let bytes = core::slice::from_raw_parts(s as *const u8, len);
            if let Ok(rust_str) = core::str::from_utf8(bytes) {
                self.insert_str(idx, rust_str);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use alloc::string::String;

    #[test]
    fn test_string_basic() {
        unsafe {
            let abi_str: AbiClass<String> =
                transmute(crate::to_abi(String::from("hello")));
            let len: usize = transmute((abi_str.methods.methods.len)(transmute(&abi_str)));
            assert_eq!(len, 5);

            let abi_str_ref: AbiClass<&str> = transmute((abi_str.methods.methods.as_str)(transmute(&abi_str)));
            let s: &str = crate::from_abi_val(transmute(abi_str_ref));
            assert_eq!(s, "hello");
        }
    }

    #[test]
    fn test_string_push() {
        unsafe {
            let mut abi_str: AbiClass<String> =
                transmute(crate::to_abi(String::from("hello")));
            (abi_str.methods.methods.push_str)(
                transmute(&mut abi_str),
                transmute(crate::to_abi(" world")),
            );
            let len: usize = transmute((abi_str.methods.methods.len)(transmute(&abi_str)));
            assert_eq!(len, 11);

            let abi_str_ref: AbiClass<&str> = transmute((abi_str.methods.methods.as_str)(transmute(&abi_str)));
            let s: &str = crate::from_abi_val(transmute(abi_str_ref));
            assert_eq!(s, "hello world");
        }
    }

    #[test]
    fn test_string_push_cstr() {
        unsafe {
            let mut abi_str: AbiClass<String> =
                transmute(crate::to_abi(String::from("hello")));
            (abi_str.methods.methods.push_cstr)(
                transmute(&mut abi_str),
                transmute(c" world".as_ptr() as *const i8),
            );
            let len: usize = transmute((abi_str.methods.methods.len)(transmute(&abi_str)));
            assert_eq!(len, 11);

            let abi_str_ref: AbiClass<&str> = transmute((abi_str.methods.methods.as_str)(transmute(&abi_str)));
            let s: &str = crate::from_abi_val(transmute(abi_str_ref));
            assert_eq!(s, "hello world");
        }
    }

    #[test]
    fn test_string_insert_str() {
        unsafe {
            let mut abi_str: AbiClass<String> = transmute(crate::to_abi(String::from("hello world")));
            (abi_str.methods.methods.insert_str)(
                transmute(&mut abi_str),
                transmute(5usize),
                transmute(crate::to_abi(" beautiful")),
            );
            let len: usize = transmute((abi_str.methods.methods.len)(transmute(&abi_str)));
            assert_eq!(len, 21);

            let abi_str_ref: AbiClass<&str> = transmute((abi_str.methods.methods.as_str)(transmute(&abi_str)));
            let s: &str = crate::from_abi_val(transmute(abi_str_ref));
            assert_eq!(s, "hello beautiful world");
        }
    }

    #[test]
    fn test_string_insert_cstr() {
        unsafe {
            let mut abi_str: AbiClass<String> = transmute(crate::to_abi(String::from("hello world")));
            (abi_str.methods.methods.insert_cstr)(
                transmute(&mut abi_str),
                transmute(5usize),
                transmute(c" beautiful".as_ptr() as *const i8),
            );
            let len: usize = transmute((abi_str.methods.methods.len)(transmute(&abi_str)));
            assert_eq!(len, 21);

            let abi_str_ref: AbiClass<&str> = transmute((abi_str.methods.methods.as_str)(transmute(&abi_str)));
            let s: &str = crate::from_abi_val(transmute(abi_str_ref));
            assert_eq!(s, "hello beautiful world");
        }
    }

    #[test]
    fn test_string_push_cstr_null() {
        unsafe {
            let mut abi_str: AbiClass<String> =
                transmute(crate::to_abi(String::from("hello")));
            (abi_str.methods.methods.push_cstr)(
                transmute(&mut abi_str),
                transmute(core::ptr::null() as *const i8),
            );
            let len: usize = transmute((abi_str.methods.methods.len)(transmute(&abi_str)));
            assert_eq!(len, 5);
        }
    }

    #[test]
    fn test_string_insert_cstr_null() {
        unsafe {
            let mut abi_str: AbiClass<String> = transmute(crate::to_abi(String::from("hello world")));
            (abi_str.methods.methods.insert_cstr)(
                transmute(&mut abi_str),
                transmute(5usize),
                transmute(core::ptr::null() as *const i8),
            );
            let len: usize = transmute((abi_str.methods.methods.len)(transmute(&abi_str)));
            assert_eq!(len, 11);
        }
    }
}
