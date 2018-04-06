# Hike
Have you ever wanted to run just one command in a different
directory? Did you have to `cd` there, do your thing and then `cd` back where
you came from? No matter what you do, it'll always take at least three commands!
Don't you find that frustrating?

*It's time your worries took a Hike!*

Jokes aside, Hike is a small program I wrote for getting to know the Rust crate
release process better. Running it with

```shell
$ hike some_dir "some --command with --arguments=and | possibly --pipes"
```

is equivalent to

```shell
$ pushd some_dir
$ some --command with --arguments=and | possibly --pipes
$ popd
```

It tries to be as transparent as possible and passes whatever stderr/stdout
output or error code the inner command produces back to your shell as if you
used `pushd`/`popd`. Using the `system()` libc function ensures that it'll
always use its enclosing shell.

Enjoy!

## Planned features
* Create the target directory (directories) if it/they don't exist
