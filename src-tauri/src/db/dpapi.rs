//! DPAPI de Windows: cifra datos ligados al usuario de Windows actual (D-007).
//! Validado en el spike `spikes/sqlcipher`.

use crate::error::AppError;

#[cfg(windows)]
pub use windows_impl::{protect, unprotect};

#[cfg(windows)]
mod windows_impl {
    use std::ptr;

    use windows_sys::Win32::Foundation::LocalFree;
    use windows_sys::Win32::Security::Cryptography::{
        CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptProtectData, CryptUnprotectData,
    };

    use super::AppError;

    pub fn protect(data: &[u8]) -> Result<Vec<u8>, AppError> {
        let input = blob_from(data);
        let mut output = empty_blob();
        // SAFETY: `input` apunta a `data`, válido durante la llamada; Windows reserva
        // `output` y `take_output` lo libera.
        let ok = unsafe {
            CryptProtectData(
                &input,
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        };
        take_output(ok, output)
    }

    pub fn unprotect(data: &[u8]) -> Result<Vec<u8>, AppError> {
        let input = blob_from(data);
        let mut output = empty_blob();
        // SAFETY: igual que en `protect`.
        let ok = unsafe {
            CryptUnprotectData(
                &input,
                ptr::null_mut(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                CRYPTPROTECT_UI_FORBIDDEN,
                &mut output,
            )
        };
        take_output(ok, output)
    }

    fn blob_from(data: &[u8]) -> CRYPT_INTEGER_BLOB {
        CRYPT_INTEGER_BLOB {
            cbData: data.len() as u32,
            pbData: data.as_ptr() as *mut u8,
        }
    }

    fn empty_blob() -> CRYPT_INTEGER_BLOB {
        CRYPT_INTEGER_BLOB {
            cbData: 0,
            pbData: ptr::null_mut(),
        }
    }

    /// Copia el resultado, borra el buffer de Windows (puede contener la clave) y lo libera.
    fn take_output(ok: i32, output: CRYPT_INTEGER_BLOB) -> Result<Vec<u8>, AppError> {
        if ok == 0 {
            return Err(AppError::Dpapi(std::io::Error::last_os_error()));
        }
        let len = output.cbData as usize;
        // SAFETY: tras éxito, Windows garantiza `cbData` bytes válidos en `pbData`.
        let bytes = unsafe { std::slice::from_raw_parts(output.pbData, len) }.to_vec();
        unsafe {
            ptr::write_bytes(output.pbData, 0, len);
            LocalFree(output.pbData as _);
        }
        Ok(bytes)
    }
}

#[cfg(not(windows))]
pub fn protect(_data: &[u8]) -> Result<Vec<u8>, AppError> {
    Err(AppError::Key("DPAPI solo existe en Windows"))
}

#[cfg(not(windows))]
pub fn unprotect(_data: &[u8]) -> Result<Vec<u8>, AppError> {
    Err(AppError::Key("DPAPI solo existe en Windows"))
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn round_trip_returns_the_original_data() {
        let secret = [7u8; 32];
        let blob = protect(&secret).unwrap();
        assert!(!blob.windows(secret.len()).any(|w| w == secret));
        assert_eq!(unprotect(&blob).unwrap(), secret);
    }

    #[test]
    fn tampered_blob_is_rejected() {
        let mut blob = protect(&[7u8; 32]).unwrap();
        let last = blob.len() - 1;
        blob[last] ^= 0xFF;
        assert!(unprotect(&blob).is_err());
    }
}
