# Encoder

Reads given input directories and encodes .mkv files with [HandBreak](https://handbrake.fr/) using options preset.json file that needs to be in same input directory as .mkv files.

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
  - Directory for encoding output files.

## Arguments

| Short | Long | Description | Default |
|---|---|---|---|
| -i | --input-dirs-path | Path to directory that contains directories with input .mkv files and preset.json file. |  |
| -o | --output-dir-path | Path to directory that encoding output files will be put. |  |
| -h | --hand-brake-cli-cmd | Command to run [HandBreak](https://handbrake.fr/) command line interface. | HandBrakeCLI |
| -c | --check-interval-seconds | How long to wait between checking for new files in seconds. | 300 |
|  | --help | Print help information |  |
| -V | --version | Print version information |  |

## Running with Docker
```sh
docker run \
    -v 'HOST_INPUT_DIR:/app/input' \
    -v 'HOST_OUTPUT_DIR:/app/output' \
    ghcr.io/santerihetekivi/encoder:master
```
