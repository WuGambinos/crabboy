use std::{
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[cfg(test)]
#[test]
fn all_cpu_blaargs() {
    // Directory Path
    let dir_path = "roms/blaargs/cpu_instrs/individual/";

    // Read the directory entries
    let mut entries: Vec<_> = fs::read_dir(dir_path)
        .expect("NOT A DIRECTORY")
        .map(|entry| entry.unwrap())
        .collect();

    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    // Iterate over the directory entries
    for entry in entries {
        let path: PathBuf = entry.path();

        // Check if it's a file
        if path.is_file() {
            run_test(path.to_str().unwrap());
        }
    }
}

fn run_test(rom_path: &str) {
    let mut emulator = Command::new("target/release/crabboy")
        .arg("--rom")
        .arg(rom_path)
        .arg("--headless")
        .arg("--skip-boot")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .expect("FAILED TO START");

    let stdout = emulator.stdout.take().expect("FAILED TO cAPTURE");
    let reader = BufReader::new(stdout);

    let path_parts: Vec<&str> = rom_path.split("/").collect();

    let file_name = path_parts[path_parts.len() - 1];

    for line in reader.lines() {
        let output = line.expect("FAILED TO READ LINE FROM EMU");
        if output.contains("Passed") {
            println!("{}: \x1B[32m{}\x1B[0m", file_name, output);

            emulator.kill().expect("COULDNT KILL");
            emulator.wait().expect("COUOLDNT WAIT");
            assert!(true);
            return;
        }
    }
    panic!("TEST FAILED");
}
