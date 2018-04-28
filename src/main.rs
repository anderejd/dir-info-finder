//!
//! Lists the modified time for the file with the latest modified time in each
//! subdirectory.
//!
#![deny(warnings)]

extern crate chrono;
#[macro_use]
extern crate log;
extern crate stderrlog;
extern crate walkdir;
use chrono::DateTime;
use chrono::Utc;
use std::env;
use std::fs;
use std::time::SystemTime;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

fn process_root(root: &Path, out: &Path) -> io::Result<()> {
    let entries = fs::read_dir(root)?;
    let mut total_gb = 0f64;
    let mut total_files = 0u64;
    let mut total_bytes = 0u64;
    let mut tuples = vec!();
    for r in entries {
        let ent = r?;
        if !ent.file_type()?.is_dir() {
            continue;
        }
        let pb = ent.path();
        info!("Scanning subdirectory: {}", &ent.path().display());
        let it = WalkDir::new(pb).into_iter().filter(|r| match *r {
            Err(_) => true,
            Ok(ref r) => r.file_type().is_file(),
        });
        let mut modified_max: Option<SystemTime> = None;
        let mut dir_bytes = 0u64;
        for r in it {
            let entry = r?;
            let meta = entry.metadata()?;
            let new_modified = meta.modified()?;
            modified_max = match modified_max {
                Some(old) => Some(old.max(new_modified)),
                None => Some(new_modified),
            };
            dir_bytes = dir_bytes.checked_add(meta.len()).unwrap();
            total_files += 1;
        }
        let dir_gb = dir_bytes as f64 / 1024f64 / 1024f64 / 1024f64;
        tuples.push((modified_max, ent.path(), dir_gb));
        total_gb += dir_gb;
        total_bytes = total_bytes.checked_add(dir_bytes).unwrap();
    }
    let outfile = File::create(out)?;
    let mut outbuf = BufWriter::new(outfile);
    writeln!(&mut outbuf, "Modified time, Directory, Total size (GB)")?;
    tuples.sort_by(|a, b| a.0.cmp(&b.0));
    for t in tuples {
        let modified_string = match t.0 {
            Some(system_time) => {
                let dt: DateTime<Utc> = system_time.into();
                let iso_8601 = "%Y-%m-%dT%H:%M:%S%:z";
                dt.format(iso_8601).to_string()
            },
            None => "                   ".into()
        };
        writeln!(
            &mut outbuf,
            "{}, {}, {}",
            modified_string,
            t.1.display(),
            t.2
        )?;
    }
    info!("Total files: {}", total_files);
    info!("Total bytes: {}", total_bytes);
    info!("Total GB:    {}", total_gb);
    Ok(())
}

fn main() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(2)
        .init()
        .unwrap();
    let root = env::args().nth(1).expect("Missing input dir as argument 1");
    let root = Path::new(root.as_str());
    let outpath = env::args()
        .nth(2)
        .expect("Missing output file path as argument 2");
    let outpath = Path::new(outpath.as_str());
    info!("Scanning root directory: {}", root.display());
    let result = process_root(root, outpath);
    match result {
        Ok(()) => (),
        Err(e) => error!("{:?}", e),
    }
}
