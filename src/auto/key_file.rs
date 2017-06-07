// This file was generated by gir (d121f7e) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v2_50")]
use Bytes;
use Error;
use KeyFileFlags;
use ffi;
use std;
use std::ptr;
use translate::*;

glib_wrapper! {
    pub struct KeyFile(Shared<ffi::GKeyFile>);

    match fn {
        ref => |ptr| ffi::g_key_file_ref(ptr),
        unref => |ptr| ffi::g_key_file_unref(ptr),
    }
}

impl KeyFile {
    pub fn new() -> KeyFile {
        unsafe {
            from_glib_full(ffi::g_key_file_new())
        }
    }

    pub fn get_boolean(&self, group_name: &str, key: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_get_boolean(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //pub fn get_boolean_list(&self, group_name: &str, key: &str) -> Result<(/*Unimplemented*/CArray TypeId { ns_id: 0, id: 1 }, /*Unimplemented*/Fundamental: Size), Error> {
    //    unsafe { TODO: call ffi::g_key_file_get_boolean_list() }
    //}

    pub fn get_comment<'a, P: Into<Option<&'a str>>>(&self, group_name: P, key: &str) -> Result<String, Error> {
        let group_name = group_name.into();
        let group_name = group_name.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_comment(self.to_glib_none().0, group_name.0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_double(&self, group_name: &str, key: &str) -> Result<f64, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_double(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    //pub fn get_double_list(&self, group_name: &str, key: &str) -> Result<(/*Unimplemented*/CArray TypeId { ns_id: 0, id: 21 }, /*Unimplemented*/Fundamental: Size), Error> {
    //    unsafe { TODO: call ffi::g_key_file_get_double_list() }
    //}

    //pub fn get_groups(&self) -> (Vec<String>, /*Unimplemented*/Fundamental: Size) {
    //    unsafe { TODO: call ffi::g_key_file_get_groups() }
    //}

    pub fn get_int64(&self, group_name: &str, key: &str) -> Result<i64, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_int64(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_integer(&self, group_name: &str, key: &str) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_integer(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    //pub fn get_integer_list(&self, group_name: &str, key: &str) -> Result<(/*Unimplemented*/CArray TypeId { ns_id: 0, id: 14 }, /*Unimplemented*/Fundamental: Size), Error> {
    //    unsafe { TODO: call ffi::g_key_file_get_integer_list() }
    //}

    //pub fn get_keys(&self, group_name: &str) -> Result<(Vec<String>, /*Unimplemented*/Fundamental: Size), Error> {
    //    unsafe { TODO: call ffi::g_key_file_get_keys() }
    //}

    pub fn get_locale_string<'a, P: Into<Option<&'a str>>>(&self, group_name: &str, key: &str, locale: P) -> Result<String, Error> {
        let locale = locale.into();
        let locale = locale.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_locale_string(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, locale.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //pub fn get_locale_string_list<'a, P: Into<Option<&'a str>>>(&self, group_name: &str, key: &str, locale: P) -> Result<(Vec<String>, /*Unimplemented*/Fundamental: Size), Error> {
    //    unsafe { TODO: call ffi::g_key_file_get_locale_string_list() }
    //}

    pub fn get_start_group(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_key_file_get_start_group(self.to_glib_none().0))
        }
    }

    pub fn get_string(&self, group_name: &str, key: &str) -> Result<String, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_string(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //pub fn get_string_list(&self, group_name: &str, key: &str) -> Result<(Vec<String>, /*Unimplemented*/Fundamental: Size), Error> {
    //    unsafe { TODO: call ffi::g_key_file_get_string_list() }
    //}

    pub fn get_uint64(&self, group_name: &str, key: &str) -> Result<u64, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_uint64(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_value(&self, group_name: &str, key: &str) -> Result<String, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_key_file_get_value(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn has_group(&self, group_name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_key_file_has_group(self.to_glib_none().0, group_name.to_glib_none().0))
        }
    }

    pub fn has_key(&self, group_name: &str, key: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_has_key(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(feature = "v2_50")]
    pub fn load_from_bytes(&self, bytes: &Bytes, flags: KeyFileFlags) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_load_from_bytes(self.to_glib_none().0, bytes.to_glib_none().0, flags.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //pub fn load_from_data(&self, data: &str, length: /*Unimplemented*/Fundamental: Size, flags: KeyFileFlags) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::g_key_file_load_from_data() }
    //}

    pub fn load_from_file<P: AsRef<std::path::Path>>(&self, file: P, flags: KeyFileFlags) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_load_from_file(self.to_glib_none().0, file.as_ref().to_glib_none().0, flags.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn remove_comment<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, group_name: P, key: Q) -> Result<(), Error> {
        let group_name = group_name.into();
        let group_name = group_name.to_glib_none();
        let key = key.into();
        let key = key.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_remove_comment(self.to_glib_none().0, group_name.0, key.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn remove_group(&self, group_name: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_remove_group(self.to_glib_none().0, group_name.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn remove_key(&self, group_name: &str, key: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_remove_key(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_boolean(&self, group_name: &str, key: &str, value: bool) {
        unsafe {
            ffi::g_key_file_set_boolean(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, value.to_glib());
        }
    }

    //pub fn set_boolean_list(&self, group_name: &str, key: &str, list: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 1 }, length: /*Unimplemented*/Fundamental: Size) {
    //    unsafe { TODO: call ffi::g_key_file_set_boolean_list() }
    //}

    pub fn set_comment<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, group_name: P, key: Q, comment: &str) -> Result<(), Error> {
        let group_name = group_name.into();
        let group_name = group_name.to_glib_none();
        let key = key.into();
        let key = key.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_key_file_set_comment(self.to_glib_none().0, group_name.0, key.0, comment.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn set_double(&self, group_name: &str, key: &str, value: f64) {
        unsafe {
            ffi::g_key_file_set_double(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    //pub fn set_double_list(&self, group_name: &str, key: &str, list: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 21 }, length: /*Unimplemented*/Fundamental: Size) {
    //    unsafe { TODO: call ffi::g_key_file_set_double_list() }
    //}

    pub fn set_int64(&self, group_name: &str, key: &str, value: i64) {
        unsafe {
            ffi::g_key_file_set_int64(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    pub fn set_integer(&self, group_name: &str, key: &str, value: i32) {
        unsafe {
            ffi::g_key_file_set_integer(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    //pub fn set_integer_list(&self, group_name: &str, key: &str, list: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 14 }, length: /*Unimplemented*/Fundamental: Size) {
    //    unsafe { TODO: call ffi::g_key_file_set_integer_list() }
    //}

    //pub fn set_list_separator(&self, separator: /*Unimplemented*/Fundamental: Char) {
    //    unsafe { TODO: call ffi::g_key_file_set_list_separator() }
    //}

    pub fn set_locale_string(&self, group_name: &str, key: &str, locale: &str, string: &str) {
        unsafe {
            ffi::g_key_file_set_locale_string(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, locale.to_glib_none().0, string.to_glib_none().0);
        }
    }

    //pub fn set_locale_string_list(&self, group_name: &str, key: &str, locale: &str, list: &[&str], length: /*Unimplemented*/Fundamental: Size) {
    //    unsafe { TODO: call ffi::g_key_file_set_locale_string_list() }
    //}

    pub fn set_string(&self, group_name: &str, key: &str, string: &str) {
        unsafe {
            ffi::g_key_file_set_string(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, string.to_glib_none().0);
        }
    }

    //pub fn set_string_list(&self, group_name: &str, key: &str, list: &[&str], length: /*Unimplemented*/Fundamental: Size) {
    //    unsafe { TODO: call ffi::g_key_file_set_string_list() }
    //}

    pub fn set_uint64(&self, group_name: &str, key: &str, value: u64) {
        unsafe {
            ffi::g_key_file_set_uint64(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    pub fn set_value(&self, group_name: &str, key: &str, value: &str) {
        unsafe {
            ffi::g_key_file_set_value(self.to_glib_none().0, group_name.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0);
        }
    }

    //pub fn to_data(&self) -> Result<(String, /*Unimplemented*/Fundamental: Size), Error> {
    //    unsafe { TODO: call ffi::g_key_file_to_data() }
    //}
}
