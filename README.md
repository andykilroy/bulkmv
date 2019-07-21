# Overview

I wrote this simple tool to aid in changing the names of large numbers
of files.  The original set of files were inconsistently named, had
varying conventions, and I wanted to introduce a more systematic naming
convention for the whole set.

I wished to run the file paths through a script (possibly sed) that
would transform the names.  I then wanted to review the results of that
transformation before actually performing the renames.  The tool only
helps with the latter part, taking a list of files to be moved, and then
moving them to paths dictated by a second list, the result of the
transformation.

The tool takes two arguments a _srcfile_ and a _destfile_.  The
_srcfile_ contains the paths of files to be moved, one file per line.
Each line of the _destfile_ is the path the corresponding file in
_srcfile_ should be moved to.  

```bash
bulkmv srcfile destfile
```

Would be equivalent to the following pseudocode:

```
let fs <- _list of filepaths from srcfile_
let ds <- _list of filepaths from destfile_

for i in 0..(length of fs)
    mv fs[i] ds[i]
```

where the _mv_ operation ensures all directories in ds[i] are created if
they don't already exist.  The tool doesn't continue if there was a
problem moving a file.

The tool does a primitive check to see if two or more files in the
srcfile list will get renamed to the same filepath.  In other words the
tool tries to check if a file will be moved over a file that was already
moved earlier.
