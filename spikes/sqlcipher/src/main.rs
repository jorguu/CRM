//! Spike S0 (desechable): valida en Windows que
//! 1) SQLCipher compila y cifra (rusqlite + bundled-sqlcipher-vendored-openssl),
//! 2) la API de backup copia una BD cifrada,
//! 3) DPAPI protege/desprotege la clave.
//! Uso: `cargo run --release`. Sale con código 1 si alguna comprobación falla.

use rusqlite::{Connection, backup::Backup};
use std::fs;
use std::path::Path;
use std::time::Duration;

const SAMPLE_NAME: &str = "Müller Haustechnik, Zürich – Öl & Gas";

fn main() {
    let dir = std::env::temp_dir().join("crm-spike-sqlcipher");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("no se pudo crear la carpeta temporal");
    let db_path = dir.join("data.db");
    let backup_path = dir.join("backup.db");

    let key = random_key();
    let wrong_key = random_key();

    let checks: Vec<(&str, Result<String, String>)> = vec![
        ("1. SQLCipher activo (PRAGMA cipher_version)", check_cipher_version(&db_path, &key)),
        ("2. Crear BD cifrada y escribir", check_create_and_write(&db_path, &key)),
        ("3. Reabrir con la clave correcta y leer", check_reopen(&db_path, &key)),
        ("4. Clave incorrecta -> falla", expect_unreadable(&db_path, Some(&wrong_key))),
        ("5. Sin clave -> falla", expect_unreadable(&db_path, None)),
        ("6. Cabecera no es 'SQLite format 3'", check_header_encrypted(&db_path)),
        ("7. Backup API -> copia cifrada legible", check_backup(&db_path, &backup_path, &key)),
        ("8. DPAPI protect/unprotect de la clave", check_dpapi(&dir, &key)),
    ];

    let mut failed = 0;
    for (name, result) in &checks {
        match result {
            Ok(detail) => println!("[OK]   {name} — {detail}"),
            Err(err) => {
                failed += 1;
                println!("[FAIL] {name} — {err}");
            }
        }
    }
    println!("\nResultado: {} OK, {} FAIL", checks.len() - failed, failed);

    let _ = fs::remove_dir_all(&dir);
    if failed > 0 {
        std::process::exit(1);
    }
}

fn random_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    getrandom::fill(&mut key).expect("CSPRNG del sistema no disponible");
    key
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn open_with_key(path: &Path, key: &[u8; 32]) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    // PRAGMA no admite parámetros. La clave es hex generado aquí, nunca texto del usuario.
    conn.execute_batch(&format!("PRAGMA key = \"x'{}'\";", to_hex(key)))?;
    Ok(conn)
}

fn read_sample(conn: &Connection) -> rusqlite::Result<String> {
    conn.query_row("SELECT name FROM sample WHERE id = 1", [], |row| row.get(0))
}

fn check_cipher_version(db: &Path, key: &[u8; 32]) -> Result<String, String> {
    let conn = open_with_key(db, key).map_err(|e| e.to_string())?;
    let version: String = conn
        .query_row("PRAGMA cipher_version", [], |row| row.get(0))
        .map_err(|e| format!("sin cipher_version (¿no es SQLCipher?): {e}"))?;
    let provider: String = conn
        .query_row("PRAGMA cipher_provider", [], |row| row.get(0))
        .unwrap_or_else(|_| "desconocido".into());
    Ok(format!("SQLCipher {version}, proveedor {provider}"))
}

fn check_create_and_write(db: &Path, key: &[u8; 32]) -> Result<String, String> {
    let conn = open_with_key(db, key).map_err(|e| e.to_string())?;
    conn.execute_batch("CREATE TABLE sample (id INTEGER PRIMARY KEY, name TEXT NOT NULL);")
        .map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO sample (id, name) VALUES (1, ?1)", [SAMPLE_NAME])
        .map_err(|e| e.to_string())?;
    Ok("tabla + 1 fila (umlauts)".into())
}

fn check_reopen(db: &Path, key: &[u8; 32]) -> Result<String, String> {
    let conn = open_with_key(db, key).map_err(|e| e.to_string())?;
    let name = read_sample(&conn).map_err(|e| e.to_string())?;
    if name == SAMPLE_NAME {
        Ok("dato idéntico".into())
    } else {
        Err(format!("dato distinto: {name}"))
    }
}

fn expect_unreadable(db: &Path, key: Option<&[u8; 32]>) -> Result<String, String> {
    let conn = match key {
        Some(k) => match open_with_key(db, k) {
            Ok(c) => c,
            Err(e) => return Ok(format!("rechazada al abrir: {e}")),
        },
        None => Connection::open(db).map_err(|e| e.to_string())?,
    };
    match conn.query_row("SELECT count(*) FROM sqlite_master", [], |row| row.get::<_, i64>(0)) {
        Ok(n) => Err(format!("¡se pudo leer! ({n} objetos)")),
        Err(e) => Ok(format!("rechazada: {e}")),
    }
}

fn check_header_encrypted(db: &Path) -> Result<String, String> {
    let bytes = fs::read(db).map_err(|e| e.to_string())?;
    let header = bytes.get(..16).ok_or("fichero demasiado corto")?;
    if header == b"SQLite format 3\0" {
        Err("cabecera en claro: la BD NO está cifrada".into())
    } else {
        Ok(format!("{} bytes, cabecera ilegible", bytes.len()))
    }
}

fn check_backup(src_path: &Path, dst_path: &Path, key: &[u8; 32]) -> Result<String, String> {
    {
        let src = open_with_key(src_path, key).map_err(|e| e.to_string())?;
        let mut dst = open_with_key(dst_path, key).map_err(|e| e.to_string())?;
        let backup = Backup::new(&src, &mut dst).map_err(|e| e.to_string())?;
        backup
            .run_to_completion(100, Duration::ZERO, None)
            .map_err(|e| e.to_string())?;
    }
    check_header_encrypted(dst_path).map_err(|e| format!("backup: {e}"))?;
    let conn = open_with_key(dst_path, key).map_err(|e| e.to_string())?;
    let name = read_sample(&conn).map_err(|e| format!("backup ilegible: {e}"))?;
    if name != SAMPLE_NAME {
        return Err("backup con dato distinto".into());
    }
    expect_unreadable(dst_path, None).map_err(|e| format!("backup sin clave: {e}"))?;
    Ok("copia cifrada, legible solo con la clave".into())
}

fn check_dpapi(dir: &Path, key: &[u8; 32]) -> Result<String, String> {
    let blob = dpapi::protect(key)?;
    if blob.windows(key.len()).any(|w| w == key) {
        return Err("el blob contiene la clave en claro".into());
    }
    let key_file = dir.join("db.key");
    fs::write(&key_file, &blob).map_err(|e| e.to_string())?;
    let read_back = fs::read(&key_file).map_err(|e| e.to_string())?;
    let recovered = dpapi::unprotect(&read_back)?;
    if recovered != key {
        return Err("la clave recuperada no coincide".into());
    }
    let mut tampered = read_back.clone();
    let last = tampered.len() - 1;
    tampered[last] ^= 0xFF;
    if dpapi::unprotect(&tampered).is_ok() {
        return Err("DPAPI aceptó un blob manipulado".into());
    }
    Ok(format!("blob de {} bytes; ida y vuelta OK; blob manipulado rechazado", blob.len()))
}

mod dpapi {
    use std::ptr;
    use windows_sys::Win32::Foundation::LocalFree;
    use windows_sys::Win32::Security::Cryptography::{
        CRYPT_INTEGER_BLOB, CRYPTPROTECT_UI_FORBIDDEN, CryptProtectData, CryptUnprotectData,
    };

    pub fn protect(data: &[u8]) -> Result<Vec<u8>, String> {
        let input = CRYPT_INTEGER_BLOB { cbData: data.len() as u32, pbData: data.as_ptr() as *mut u8 };
        let mut output = CRYPT_INTEGER_BLOB { cbData: 0, pbData: ptr::null_mut() };
        // SAFETY: los punteros son válidos durante la llamada; Windows reserva `output`
        // y lo liberamos con LocalFree tras copiarlo.
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
        take_output(ok, output, "CryptProtectData")
    }

    pub fn unprotect(data: &[u8]) -> Result<Vec<u8>, String> {
        let input = CRYPT_INTEGER_BLOB { cbData: data.len() as u32, pbData: data.as_ptr() as *mut u8 };
        let mut output = CRYPT_INTEGER_BLOB { cbData: 0, pbData: ptr::null_mut() };
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
        take_output(ok, output, "CryptUnprotectData")
    }

    fn take_output(ok: i32, output: CRYPT_INTEGER_BLOB, func: &str) -> Result<Vec<u8>, String> {
        if ok == 0 {
            return Err(format!("{func} falló: {}", std::io::Error::last_os_error()));
        }
        // SAFETY: tras éxito, Windows garantiza `cbData` bytes válidos en `pbData`.
        let bytes = unsafe { std::slice::from_raw_parts(output.pbData, output.cbData as usize) }.to_vec();
        unsafe { LocalFree(output.pbData as _) };
        Ok(bytes)
    }
}
