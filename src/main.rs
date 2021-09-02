// Clap for parsing command line arguments.
use clap::{AppSettings, Clap};

// fs for file system actions.
// thread and time for sleeping.
use std::{fs, thread, time};

// Path for building and storing paths.
use std::path::{Path, PathBuf};

// Operating system strings for storing them.
use std::ffi::OsStr;

// Command for running commands and Stdio for output.
use std::process::{Command, Stdio};

// Clamp config for command line arguments.
#[derive(Clap)]
#[clap(version = "1.0", author = "Santeri Hetekivi <santeri@hetekivi.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
// Command line arguments struct.
struct Opts {
    #[clap(
        short,
        long,
        about = "Path to directory that contains directories with input .mkv files and preset.json file."
    )]
    input_dirs_path: String,
    #[clap(
        short,
        long,
        about = "Path to directory that encoding output files will be put."
    )]
    output_dir_path: String,
    #[clap(
        short,
        long,
        default_value = "HandBrakeCLI",
        about = "Command to run HandBrake command line interface."
    )]
    hand_brake_cli_cmd: String,
    #[clap(
        short,
        long,
        default_value = "300",
        about = "How long to wait between checking for new files in seconds."
    )]
    check_interval_seconds: i32,
}

// Struct for encode job data.
struct EncodeJob {
    // Input file's path buffer.
    input: PathBuf,
    // Preset file's path buffer.
    preset: PathBuf,
}

/// Get encode job data.
///
/// Return result with optional EncodeJob struct;
fn get_encode_job() -> Result<Option<EncodeJob>, std::io::Error> {
    // Init input and preset paths.
    let mut input_path: Option<PathBuf>;
    let mut preset_path: Option<PathBuf>;

    // Get input directories path from command line arguments.
    let input_dirs_path: String = Opts::parse().input_dirs_path;

    // Start reading entries in input directories path.
    println!("Reading path {}...", input_dirs_path);
    for entry in fs::read_dir(input_dirs_path)? {
        // Get path for entry.
        let path: PathBuf = entry?.path();
        println!("\tReading entry {:?}...", path.file_name().unwrap());

        // If entry is not directory
        if !path.is_dir() {
            println!("\t\tWas not directory!");
            // continue to next entry.
            continue;
        }
        // Entry was directory.
        println!("\t\tWas directory.");

        // Clear preset and input paths.
        preset_path = None;
        input_path = None;

        // Read entries from current entry that was directory.
        println!("\t\tReading files...");
        for sub_entry in fs::read_dir(path)? {
            // Get path
            let sub_path = sub_entry?.path();
            // and filename from entry.
            let file_name: &str = sub_path.file_name().unwrap().to_str().unwrap();
            println!("\t\t\tReading entry {}...", file_name);
            let extension: Option<&OsStr> = sub_path.extension();

            // If entry was not file
            if !sub_path.is_file() {
                println!("\t\t\t\tWas not file!");
                // continue to the next.
                continue;
            }
            // If entry was preset file
            else if file_name == "preset.json" {
                println!("\t\t\t\tWas preset file!");
                // set preset file.
                preset_path = Some(sub_path);

                // If input path also found
                if input_path.is_some() {
                    // Return with EncodeJob data.
                    return Ok(Some(EncodeJob {
                        input: input_path.unwrap(),
                        preset: preset_path.unwrap(),
                    }));
                }
            }
            // If entry was mkv file, so input file
            else if extension.is_some() && extension.unwrap() == "mkv" {
                println!("\t\t\t\tWas input file!");
                // set input path.
                input_path = Some(sub_path);
                // If preset path also found
                if preset_path.is_some() {
                    // Return with EncodeJob data.
                    return Ok(Some(EncodeJob {
                        input: input_path.unwrap(),
                        preset: preset_path.unwrap(),
                    }));
                }
            }
            // File was not preset or input -file.
            else {
                println!("\t\t\t\tWas unknown file!");
            }
        }
    }

    // No directory contained preset and input files.
    println!("No files for encoding found!");
    // Return with none.
    return Ok(None);
}

/// Encode given input file with given preset.
///
/// # Arguments
///
/// * `preset_file` - PatBuf of preset file to use.
/// * `input_file` - PatBuf of input file to use.
///
/// Return empty result or String error.
fn encode(preset_file: PathBuf, input_file: PathBuf) -> Result<(), String> {
    // Parse command line arguments.
    let opts: Opts = Opts::parse();
    // Get input file name.
    let input_file_name: &str = input_file.file_name().unwrap().to_str().unwrap();
    // Get output directory path.
    let output_dir_path: String = opts.output_dir_path;
    // Generate output path buffer from output directory path and input filename.
    let output_path_buffer: PathBuf = Path::new(&output_dir_path).join(input_file_name);
    let output: &str = output_path_buffer.to_str().unwrap();
    // If output file already exits.
    if output_path_buffer.is_file() {
        // return error.
        return Err(format!("Output path is already file '{}'!", output));
    }

    // Start to encode with given file.
    println!("Starting to encode file {}...", input_file_name);
    // Run command and get result.
    let result = Command::new(opts.hand_brake_cli_cmd)
        .arg("--preset-import-file")
        .arg(preset_file.to_str().unwrap())
        .arg("-i")
        .arg(input_file.to_str().unwrap())
        .arg("-o")
        .arg(output)
        .stdout(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    // If command failed
    if !result.success() {
        // return error.
        return Err(format!(
            "Encoding failed with code {}!",
            result.code().unwrap()
        ));
    }
    // If no output file was not generated!
    else if !output_path_buffer.is_file() {
        // return error.
        return Err(format!(
            "Encoding did not produce output file '{}'!",
            output
        ));
    }
    // Encoding succeeded.
    println!("Encoding succeeded!");

    // Remove input file.
    fs::remove_file(input_file).unwrap();
    println!("Input file removed!");

    // Return success.
    return Ok(());
}

// Main loop.
fn main() -> std::io::Result<()> {
    // Get sleep seconds from command line argument.
    let sleep_seconds: i32 = Opts::parse().check_interval_seconds;
    // Generate sleep duration from sleep interval seconds.
    let sleep_duration: std::time::Duration = time::Duration::from_secs_f32(sleep_seconds as f32);
    // Loop indefinitely.
    loop {
        // Get encode job data.
        let encode_job: Option<EncodeJob> = get_encode_job()?;
        // Data found.
        if encode_job.is_some() {
            let encode_job: EncodeJob = encode_job.unwrap();
            // Start encoding with the data.
            encode(encode_job.preset, encode_job.input).unwrap();
        }
        // No data found.
        else {
            // Sleep for given duration before next check.
            println!("Sleeping {} seconds...", sleep_seconds);
            thread::sleep(sleep_duration);
        }
    }
}
