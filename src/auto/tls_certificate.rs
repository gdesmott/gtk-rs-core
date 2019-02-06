// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use SocketConnectable;
use TlsCertificateFlags;
use ffi;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct TlsCertificate(Object<ffi::GTlsCertificate, ffi::GTlsCertificateClass, TlsCertificateClass>);

    match fn {
        get_type => || ffi::g_tls_certificate_get_type(),
    }
}

impl TlsCertificate {
    pub fn new_from_file<P: AsRef<std::path::Path>>(file: P) -> Result<TlsCertificate, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_file(file.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_from_files<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(cert_file: P, key_file: Q) -> Result<TlsCertificate, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_files(cert_file.as_ref().to_glib_none().0, key_file.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_from_pem(data: &str) -> Result<TlsCertificate, Error> {
        let length = data.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_pem(data.to_glib_none().0, length, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn list_new_from_file<P: AsRef<std::path::Path>>(file: P) -> Result<Vec<TlsCertificate>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_list_new_from_file(file.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

pub const NONE_TLS_CERTIFICATE: Option<&TlsCertificate> = None;

pub trait TlsCertificateExt: 'static {
    fn get_issuer(&self) -> Option<TlsCertificate>;

    fn is_same<P: IsA<TlsCertificate>>(&self, cert_two: &P) -> bool;

    fn verify<'a, 'b, P: IsA<SocketConnectable> + 'a, Q: Into<Option<&'a P>>, R: IsA<TlsCertificate> + 'b, S: Into<Option<&'b R>>>(&self, identity: Q, trusted_ca: S) -> TlsCertificateFlags;

    //fn get_property_certificate(&self) -> /*Ignored*/Option<glib::ByteArray>;

    fn get_property_certificate_pem(&self) -> Option<GString>;
}

impl<O: IsA<TlsCertificate>> TlsCertificateExt for O {
    fn get_issuer(&self) -> Option<TlsCertificate> {
        unsafe {
            from_glib_none(ffi::g_tls_certificate_get_issuer(self.as_ref().to_glib_none().0))
        }
    }

    fn is_same<P: IsA<TlsCertificate>>(&self, cert_two: &P) -> bool {
        unsafe {
            from_glib(ffi::g_tls_certificate_is_same(self.as_ref().to_glib_none().0, cert_two.as_ref().to_glib_none().0))
        }
    }

    fn verify<'a, 'b, P: IsA<SocketConnectable> + 'a, Q: Into<Option<&'a P>>, R: IsA<TlsCertificate> + 'b, S: Into<Option<&'b R>>>(&self, identity: Q, trusted_ca: S) -> TlsCertificateFlags {
        let identity = identity.into();
        let trusted_ca = trusted_ca.into();
        unsafe {
            from_glib(ffi::g_tls_certificate_verify(self.as_ref().to_glib_none().0, identity.map(|p| p.as_ref()).to_glib_none().0, trusted_ca.map(|p| p.as_ref()).to_glib_none().0))
        }
    }

    //fn get_property_certificate(&self) -> /*Ignored*/Option<glib::ByteArray> {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"certificate\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get()
    //    }
    //}

    fn get_property_certificate_pem(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"certificate-pem\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }
}

impl fmt::Display for TlsCertificate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TlsCertificate")
    }
}
