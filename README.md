# Encoder

Reads given input directories and encodes .mkv files with HandBreak using options preset.json file that needs to be in same input directory as .mkv files.

## Example input file structure:
- input (directory)
  - bluray (directory)
    - preset.json (preset file to use for all files in this bluray directory)
    - test.mkv (encode this with options from preset.json file, that is in this bluray directory)
    - test2.mkv (encode this with options from preset.json file, that is in this bluray directory)
  - dvd (directory)
    - preset.json (preset file to use for all files in this dvd directory)
    - test3.mkv (encode this with options from preset.json file, that is in this dvd directory)
    - test4.mkv (encode this with options from preset.json file, that is in this dvd directory)
- output (directory)
  - ALL ENCODED FILES MOVE HERE!

## Arguments

| Short | Long | Description | Default |
|---|---|---|---|
| -i | --input-dirs-path | Path to directory that contains directories with input .mkv files and preset.json file. |  |
| -o | --output-dir-path | Path to directory that encoding output files will be put. |  |
| -h | --hand-brake-cli-cmd | Command to run HandBrake command line interface. | HandBrakeCLI |
| -c | --check-interval-seconds | How long to wait between checking for new files in seconds. | 300 |
|  | --help | Print help information |  |
| -V | --version | Print version information |  |
