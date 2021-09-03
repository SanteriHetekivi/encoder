// Clap for parsing command line arguments.
use clap::{AppSettings, Clap};

// fs for file system actions.
// thread and time for sleeping.
use std::{fs, thread, time};

// Path for building and storing paths.
use std::path::{Path, PathBuf};

// Command for running commands and Stdio for output.
use std::process::{Command, ExitStatus, Stdio};

// Generate custom error.
use std::error::Error;
use std::fmt;
#[derive(Debug)]
struct MyError(String);
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl Error for MyError {}

// Clamp config for command line arguments.
#[derive(Clap)]
#[clap(version = "1.0.1", author = "Santeri Hetekivi <santeri@hetekivi.com>")]
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
fn get_encode_job() -> Result<Option<EncodeJob>, Box<dyn Error>> {
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
        println!(
            "\tReading entry {}...",
            match path.file_name() {
                Some(os_filename) => match os_filename.to_str() {
                    Some(filename) => filename,
                    None => "TRANSFORMING_OS_FILENAME_TO_STR_FAILED",
                },
                None => "GETTING_OS_FILENAME_FAILED",
            }
        );

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
            let file_name: String = match (match sub_path.file_name() {
                // Succeeded to get filename from input file.
                Some(input_file_name) => input_file_name,
                // Failed to get filename from input file!
                None => {
                    return Err(Box::new(MyError(
                        "Failed to get filename from PathBug sub_path!".into(),
                    )));
                }
            })
            .to_str()
            {
                // Succeeded to transform input filename to str.
                Some(input_file_name) => input_file_name.into(),
                // Failed to transform input filename to str!
                None => {
                    return Err(Box::new(MyError(
                        "Failed to transform PathBuf sub_path to str!".into(),
                    )));
                }
            };
            println!("\t\t\tReading entry {}...", file_name);
            // Get extension as String.
            let extension: String = match sub_path.extension() {
                Some(os_path) => match os_path.to_str() {
                    Some(string) => string.into(),
                    None => "".into(),
                },
                None => "".into(),
            };

            // If entry was not file
            if !sub_path.is_file() {
                println!("\t\t\t\tWas not file!");
                // continue to the next.
                continue;
            }
            // If entry was preset file
            else if file_name == "preset.json" {
                println!("\t\t\t\tWas preset file!");
                match input_path {
                    // If input path also found
                    Some(input_path) => {
                        // Return with EncodeJob data.
                        return Ok(Some(EncodeJob {
                            input: input_path,
                            preset: sub_path,
                        }));
                    }
                    None => {}
                }
                // set preset file.
                preset_path = Some(sub_path);
            }
            // If entry was mkv file, so input file
            else if extension == "mkv" {
                println!("\t\t\t\tWas input file!");
                // set input path.
                match preset_path {
                    // If preset path also found
                    Some(preset_path) => {
                        // Return with EncodeJob data.
                        return Ok(Some(EncodeJob {
                            input: sub_path,
                            preset: preset_path,
                        }));
                    }
                    None => {}
                }
                input_path = Some(sub_path);
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
fn encode(preset_file: PathBuf, input_file: PathBuf) -> Result<(), Box<dyn Error>> {
    // Parse command line arguments.
    let opts: Opts = Opts::parse();
    // Get input file name.
    let input_file_name: &str = match (match input_file.file_name() {
        // Succeeded to get filename from input file.
        Some(input_file_name) => input_file_name,
        // Failed to get filename from input file!
        None => {
            return Err(Box::new(MyError(
                "Failed to get filename from PathBug input_file_name!".into(),
            )));
        }
    })
    .to_str()
    {
        // Succeeded to transform input filename to str.
        Some(string) => string,
        // Failed to transform input filename to str!
        None => {
            return Err(Box::new(MyError(
                "Failed to transform PathBuf input_file_name to str!".into(),
            )));
        }
    };
    // Generate output path buffer from output directory path and input filename.
    let output_path_buffer: PathBuf = Path::new(&opts.output_dir_path).join(input_file_name);
    let output: &str = match output_path_buffer.to_str() {
        // Succeeded to transform input filename to str.
        Some(string) => string,
        // Failed to transform input filename to str!
        None => {
            return Err(Box::new(MyError(
                "Failed to transform PathBuf output_path_buffer to str!".into(),
            )));
        }
    };
    // If output file already exits.
    if output_path_buffer.clone().is_file() {
        // return error.
        return Err(Box::new(MyError(format!(
            "Output path is already file '{}'!",
            output
        ))));
    }

    // Start to encode with given file.
    println!("Starting to encode file {}...", input_file_name);
    // Run command and get result.
    let result: ExitStatus = match (match Command::new(opts.hand_brake_cli_cmd)
        .arg("--preset-import-file")
        .arg(match preset_file.to_str() {
            // Succeeded to transform input filename to str.
            Some(string) => string,
            // Failed to transform input filename to str!
            None => {
                return Err(Box::new(MyError(
                    "Failed to transform PathBuf preset_file to str!".into(),
                )));
            }
        })
        .arg("-i")
        .arg(match input_file.to_str() {
            // Succeeded to transform input filename to str.
            Some(string) => string,
            // Failed to transform input filename to str!
            None => {
                return Err(Box::new(MyError(
                    "Failed to transform PathBuf input_file to str!".into(),
                )));
            }
        })
        .arg("-o")
        .arg(output)
        .stdout(Stdio::inherit())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => return Err(Box::new(err)),
    })
    .wait()
    {
        Ok(result) => result,
        Err(err) => return Err(Box::new(err)),
    };
    // If command failed
    if !result.success() {
        // return error.
        match result.code() {
            Some(code) => {
                return Err(Box::new(MyError(format!(
                    "Encoding failed with code {}!",
                    code
                ))));
            }
            None => {
                return Err(Box::new(MyError("Encoding failed without code!".into())));
            }
        }
    }
    // If no output file was not generated!
    else if !output_path_buffer.is_file() {
        // return error.
        return Err(Box::new(MyError(format!(
            "Encoding did not produce output file '{}'!",
            output
        ))));
    }
    // Encoding succeeded.
    println!("Encoding succeeded!");

    // Remove input file.
    match fs::remove_file(input_file) {
        Ok(ok) => ok,
        Err(error) => return Err(Box::new(error)),
    }
    println!("Input file removed!");

    // Return success.
    return Ok(());
}

fn run() -> Result<(), Box<dyn Error>> {
    // Get sleep seconds from command line argument.
    let sleep_seconds: i32 = Opts::parse().check_interval_seconds;
    // Generate sleep duration from sleep interval seconds.
    let sleep_duration: std::time::Duration = time::Duration::from_secs_f32(sleep_seconds as f32);
    // Loop indefinitely.
    loop {
        // Get encode job data.
        match get_encode_job()? {
            // Data found.
            Some(encode_job) => {
                // Start encoding with the data.
                encode(encode_job.preset, encode_job.input)?;
            }
            // No data found.
            None => {
                // Sleep for given duration before next check.
                println!("Sleeping {} seconds...", sleep_seconds);
                thread::sleep(sleep_duration);
            }
        }
    }
}

// Main function
fn main() {
    // Run
    match run() {
        Ok(_) => {
            println!("Done!");
            std::process::exit(exitcode::OK);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(exitcode::DATAERR);
        }
    }
}
