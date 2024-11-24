use std::ffi::{c_void, CString};

#[repr(C)]
pub struct TypeInfo {
    pub label: *const u8,

    pub mime_type: *const u8,

    pub group: *const u8,

    pub description: *const u8,

    pub extensions_length: usize,
    pub extensions: *const *const u8,

    pub is_text: u8,
}

impl From<&magika::TypeInfo> for TypeInfo {
    fn from(info: &magika::TypeInfo) -> Self {
        let exts = info
            .extensions
            .iter()
            .map(|&ext| CString::new(ext).unwrap().into_raw() as *const u8)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self {
            label: CString::new(info.label).unwrap().into_raw() as *const u8,
            mime_type: CString::new(info.mime_type).unwrap().into_raw() as *const u8,
            group: CString::new(info.group).unwrap().into_raw() as *const u8,
            description: CString::new(info.description).unwrap().into_raw() as *const u8,
            extensions_length: exts.len(),
            extensions: Box::into_raw(exts) as *const *const u8,
            is_text: info.is_text as u8,
        }
    }
}

#[repr(C)]
pub struct ResultWrap<T> {
    pub pointer: *mut T,
    pub error: usize,
    pub error_message: *const u8,
}

#[repr(usize)]
pub enum Error {
    CannotCreateSession = 1,
    CannotIdentifyFile,
    NullSession,
    InvalidFilePathToIdentify,
}

#[no_mangle]
pub extern "C" fn magika_session_new() -> *const ResultWrap<c_void> {
    let session = magika::Session::new();

    let mut result = ResultWrap {
        pointer: std::ptr::null_mut(),
        error: 0,
        error_message: std::ptr::null_mut(),
    };

    match session {
        Ok(session) => result.pointer = Box::into_raw(Box::new(session)) as *mut c_void,
        Err(err) => {
            let err = CString::new(err.to_string()).unwrap();
            result.error_message = err.into_raw() as *const u8;
            result.error = Error::CannotCreateSession as usize;
        }
    }

    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub extern "C" fn magika_session_free(session: *mut c_void) {
    if session.is_null() {
        return;
    }

    unsafe {
        let result_wrapper = Box::from_raw(session as *mut ResultWrap<c_void>);
        let _ = Box::from_raw(result_wrapper.pointer as *mut magika::Session);
        if !result_wrapper.error_message.is_null() {
            let _ = CString::from_raw(result_wrapper.error_message as *mut i8);
        }
    }
}

#[no_mangle]
pub extern "C" fn magika_identify_file_sync(
    session: *mut c_void,
    path: *const u8,
    path_len: usize,
) -> *const ResultWrap<TypeInfo> {
    let mut result = ResultWrap {
        pointer: std::ptr::null_mut(),
        error: 0,
        error_message: std::ptr::null_mut(),
    };

    if session.is_null() {
        result.error = Error::NullSession as usize;
        return Box::into_raw(Box::new(result));
    }

    let session = unsafe { &*(session as *mut magika::Session) };

    let path = unsafe { std::slice::from_raw_parts(path, path_len) };
    let path = match std::str::from_utf8(path) {
        Ok(path) => path,
        Err(err) => {
            let err = CString::new(err.to_string()).unwrap();
            result.error_message = err.into_raw() as *const u8;
            result.error = Error::InvalidFilePathToIdentify as usize;
            return Box::into_raw(Box::new(result));
        }
    };

    match session.identify_file_sync(path) {
        Ok(file_type) => {
            let info: TypeInfo = file_type.info().into();
            result.pointer = Box::into_raw(Box::new(info));
        }
        Err(err) => {
            let err = CString::new(err.to_string()).unwrap();
            result.error_message = err.into_raw() as *const u8;
            result.error = Error::CannotIdentifyFile as usize;
            return Box::into_raw(Box::new(result));
        }
    }

    Box::into_raw(Box::new(result))
}

#[no_mangle]
pub extern "C" fn magika_type_info_free(result_wrapper: *mut c_void) {
    if result_wrapper.is_null() {
        return;
    }

    unsafe {
        let result = Box::from_raw(result_wrapper as *mut ResultWrap<TypeInfo>);

        if !result.error_message.is_null() {
            let _ = CString::from_raw(result.error_message as *mut i8);
        }

        let typeinfo = Box::from_raw(result.pointer as *mut TypeInfo);

        if !typeinfo.label.is_null() {
            let _ = CString::from_raw(typeinfo.label as *mut i8);
        }

        if !typeinfo.mime_type.is_null() {
            let _ = CString::from_raw(typeinfo.mime_type as *mut i8);
        }

        if !typeinfo.group.is_null() {
            let _ = CString::from_raw(typeinfo.group as *mut i8);
        }

        if !typeinfo.description.is_null() {
            let _ = CString::from_raw(typeinfo.description as *mut i8);
        }

        if !typeinfo.extensions.is_null() {
            let vec_str = Vec::from_raw_parts(
                typeinfo.extensions as *mut *const i8,
                typeinfo.extensions_length,
                typeinfo.extensions_length,
            );

            for s_ptr in vec_str {
                let _ = CString::from_raw(s_ptr as *mut i8);
            }
        }
    }
}
