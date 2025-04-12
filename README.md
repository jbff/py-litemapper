# LiteMapper (Python Port)

A command-line tool for automatically adding lighting events to Beat Saber beatmaps.

## Overview

This project is a Python port of [LiteMapper](https://github.com/itsorius/LiteMapper), based on commit ef21fe1478e6c8589d8aaced009fa19c3f9c47f2. It converts the original JavaScript web application into a command-line tool that can automatically add lighting events to Beat Saber beatmaps.

## Features

- Automatically adds lighting events to Beat Saber beatmaps
- Command-line interface for easy batch processing
- Support for overwriting existing lighting events
- Option to replace input files directly

## Installation

1. Clone this repository:
```bash
git clone https://github.com/jbff/py-litemapper.git
cd py-litemapper
```

2. Ensure you have Python 3.x installed

## Usage

Basic usage:
```bash
python litemapper.py -i input.dat -o output.dat
```

### Command Line Options

- `-i, --input`: Input beatmap file (required)
- `-o, --output`: Output beatmap file (required if --replace is not used)
- `-r, --replace`: Overwrite the input file with the new beatmap (cannot be used with --output)
- `-f, --force`: Force overwrite existing lighting events

### Examples

1. Create a new beatmap with lighting:
```bash
python litemapper.py -i Hard.dat -o HardLights.dat
```

2. Replace existing lighting events:
```bash
python litemapper.py -i Hard.dat -o HardLights.dat -f
```

3. Overwrite the input file:
```bash
python litemapper.py -i Hard.dat -r
```

## Development History

This project was created through a series of AI-assisted conversions and improvements:

1. Initial conversion from JavaScript to Python using Cursor AI editor and LLMs
2. Bug fixes and error corrections by Warp AI terminal
3. Addition of `--force` option to handle existing lighting events
4. Implementation of `--replace` functionality for in-place updates
5. Verification and correction of logic to match the original JavaScript implementation

## License

This project is based on LiteMapper by itsorius. Please refer to the original project's license for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- Original [LiteMapper](https://github.com/itsorius/LiteMapper) project by [itsorius](https://github.com/itsorius)
- [Cursor AI](https://cursor.sh/) editor and LLMs for initial conversion
- [Warp AI](https://www.warp.dev/) terminal for bug fixes and improvements 