py-litemapper
=============

* litemapper.py is a port of https://github.com/itsorius/LiteMapper
* It is based on commit ef21fe1478e6c8589d8aaced009fa19c3f9c47f2
* It was converted from JavaScript by the Cursor AI editor and LLMs.

The prompt given to convert LiteMapper into py-litemapper was:
Please write a python script based on this nodejs application. Rather than create a web app like this app is, I want you to take the LiteMapper functionality in public/js/index.js and index.js and create a python script that takes a Beat Saber beatmap as an argument on the command line, loads it, then adds lights to the map automatically using the logic in public/js/index.js, then it should save the new beatmap with lights to a new filename (also specified on command line). so for example, you would run litemapper.py -i Hard.dat -o HardLights.dat

* LLM was set to Auto and it is unknown which LLM was used to do the conversion.

* There was an array error when first running it. the Warp AI terminal offered to fix the error and I let it do so. The script works great after it made its changes.

* Then I decided to add another command line and back in Cursor:

* claude-3.7-sonnet-max was given some editing instructions:
I would like to add a command line option --force that will drop old _events already existing and go ahead and do the auto lighting.  Normal behavior is if _events is empty, go ahead and do the lighting. If _events contains events, normal behavior is to say beatmap already contains lighting events and then exit. But if --force or -f is given at command line, if _events contains events, it will ignore the _events existing in the file, do the lighting as if there were no previous lighting events, and then save the file with the new lighting.

* With the following additional prompt:
It isn't enough to test if _events exists, but test to see if it is empty; if it does not exist or it exists but is empty, then it has no lighting; if it exists and is not empty, it has lighting, and --force is required to run the autolighting.

* After some testing, I gave the Warp shell this prompt:
compare the logic in litemapper.py to the logic in public/js/index.js and ensure the python version is a precise replica of the javascript logic. the output from the python version and the javascript version are very similar but not the same, so I just want to check to see if there is any place in the python script that might be doing something slightly different from the javascript version.

* And it found and corrected several issues autonomously after creating a plan with o1.

The result of this process is the litemapper.py you find here. Now there is a simple command line version of litemapper that is much easier to use to batch process files than the javascript web app.
