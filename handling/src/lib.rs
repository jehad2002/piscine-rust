use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(file: P, content: &str) {
    let mut f_h = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file.as_ref())
    {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    };

    if let Err(err) = f_h.write_all(content.as_bytes()) {
        panic!("{}", err);
    }
}
//._.