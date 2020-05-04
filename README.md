# csdeps
Given a directory, csdeps recursively searches folders for `.csproj` files, and prints out all dependencies.
* Use the `json` flag to print output in that format.

```
USAGE:
    csdeps [FLAGS] <DIR>

FLAGS:
    -h, --help       Prints help information
    -j, --json       Output using json format
    -V, --version    Prints version information

ARGS:
    <DIR>    Directory to search for project files
```
