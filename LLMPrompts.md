# LiteMapper Development History and LLM Prompts

This document details the development history of the LiteMapper Python port and the LLM prompts used throughout its creation.

## Project Origin

This project is a Python port of [LiteMapper](https://github.com/itsorius/LiteMapper), based on commit ef21fe1478e6c8589d8aaced009fa19c3f9c47f2. The original JavaScript web application was converted into a command-line tool using AI assistance.

## Development Timeline

### 1. Initial Conversion to Python

**Prompt:**
```
Please write a python script based on this nodejs application. Rather than create a web app like this app is, I want you to take the LiteMapper functionality in public/js/index.js and index.js and create a python script that takes a Beat Saber beatmap as an argument on the command line, loads it, then adds lights to the map automatically using the logic in public/js/index.js, then it should save the new beatmap with lights to a new filename (also specified on command line). so for example, you would run litemapper.py -i Hard.dat -o HardLights.dat
```

**Details:**
- LLM was set to Auto (unknown which specific model was used)
- Initial conversion resulted in an array error
- Warp AI terminal offered to fix the error and successfully did so

### 2. Addition of --force Option

**Prompt to claude-3.7-sonnet-max:**
```
I would like to add a command line option --force that will drop old _events already existing and go ahead and do the auto lighting. Normal behavior is if _events is empty, go ahead and do the lighting. If _events contains events, normal behavior is to say beatmap already contains lighting events and then exit. But if --force or -f is given at command line, if _events contains events, it will ignore the _events existing in the file, do the lighting as if there were no previous lighting events, and then save the file with the new lighting.
```

**Follow-up Prompt:**
```
It isn't enough to test if _events exists, but test to see if it is empty; if it does not exist or it exists but is empty, then it has no lighting; if it exists and is not empty, it has lighting, and --force is required to run the autolighting.
```

### 3. Logic Verification

**Prompt to Warp Shell:**
```
compare the logic in litemapper.py to the logic in public/js/index.js and ensure the python version is a precise replica of the javascript logic. the output from the python version and the javascript version are very similar but not the same, so I just want to check to see if there is any place in the python script that might be doing something slightly different from the javascript version.
```

**Result:**
- Warp AI found and corrected several issues autonomously
- Created a plan with o1 to verify and fix discrepancies
- Resulted in a more accurate Python implementation matching the JavaScript logic

### 4. Addition of --replace Functionality

**Prompt:**
```
Add a command line option --replace or -r that will overwrite the input file with the output file. --output and --replace cannot both be used at the same time. and one or the other of them must always be used. if --output is used, it writes to the output file specified. if --replace is used, it will overwrite the input file with the new file. you can choose how to handle the replace, either directly writing to the original file, or writing to a temp file and then replacing the original file with it.
```

## Final Result

The result of this development process is a command-line version of LiteMapper that:
- Maintains the same lighting logic as the original JavaScript version
- Provides a simpler interface for batch processing files
- Includes additional features like force overwrite and file replacement
- Is more suitable for automated processing than the original web application

## Tools Used

- [Cursor AI](https://cursor.sh/) editor and LLMs for initial conversion
- [Warp AI](https://www.warp.dev/) terminal for bug fixes and improvements
- Original [LiteMapper](https://github.com/itsorius/LiteMapper) project by [itsorius](https://github.com/itsorius) 