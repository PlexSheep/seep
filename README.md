# seep

Print `stdin` to terminal, then pipe into next process.

`seep` (short for see pipe and also to describe leaks in real pipes) has the
purpose of letting you peek at what you're piping.

## Usage

On Unix like systems, you can pass the output (`stdout`) of one process to the
other as input, like this: `echo "foo" | hexdump`. In some cases, the output of
the first command might contain information that a user might want to look at.

When the second process does not show the information it received, the user
cannot *see* the information produced by the first program. This is where `seep`
comes useful:

To look at the output of process one, we pipe it to `seep` and then pipe the
output of `seep` to process two. `seep` will show us what information it
receives and pass it over to process two:

```bash
$ ls | seep | grep src
Cargo.lock
Cargo.toml
LICENSE
README.md
scripts
src
target

src
```
(list files and dirs, show all with `seep`, show only containing "src")

## Similarity to `tee`

The command `tee` is part of the coreutils and available on almost any Unix like
system. It can be used to achieve similar things as `seep`, for example:

```bash
$ ls | tee $TTY | grep src
Cargo.lock
Cargo.toml
LICENSE
README.md
scripts
src
target
src
```
(list files and dirs, show all with `tee`, show only containing "src")

`tee` and `seep` do not have the same features. Currently, `seep` cannot output
to files specified with cli arguments, and `seep`'s focus lies on presenting
information to the user.
