dir-info-finder
===============

A program that finds and lists the latest modified timestamp for the latest
modified file in each subdirectory. Useful for finding old data that can be
archived or deleted.

Logs to `stderr`.

Syntax
------

dir-info-finder "root directory path" "output CSV file path"

Example
-------

Command:
```
dir-info-finder ~/code/dir-info-finder/ /dev/stdout 
```

stderr:
```
INFO - Scanning root directory: /Users/u/code/dir-info-finder/
INFO - Scanning subdirectory: /Users/u/code/dir-info-finder/target
INFO - Scanning subdirectory: /Users/u/code/dir-info-finder/.git
INFO - Scanning subdirectory: /Users/u/code/dir-info-finder/src
INFO - Total files: 143
INFO - Total bytes: 8905834
INFO - Total GB:    0.008294204249978065
```

stdout:
```
Modified time, Directory, Total size (GB)
2018-04-28T11:03:56+00:00, /Users/u/code/dir-info-finder/src, 0.0000029653310775756836
2018-04-28T19:20:27+00:00, /Users/u/code/dir-info-finder/.git, 0.00003574881702661514
2018-04-28T19:20:37+00:00, /Users/u/code/dir-info-finder/target, 0.008255490101873875
```

