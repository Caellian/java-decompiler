# Java Decompiler

[![Build](https://github.com/Caellian/java-decompiler/workflows/Build/badge.svg)](https://github.com/Caellian/java-decompiler/actions?query=workflow%3A%22Build%22)

A modern and very efficient Java decompiler.

## Developement

It can read a decompile class files.
Pattern matching doesn't cover all instructions yet so a lot of the output is commented out assembly.

Most command line arguments aren't properly handled yet.

## JVM support status

Latest version is currently being written to support any semi-recent Java version.
At a later stage, I'd like to go through [specs](https://docs.oracle.com/javase/specs/) and allow targetting different versions better.

## License

This project is licensed under the GPL license, version 3.
A copy of the GPL license is provided in the [LICENSE.md](LICENSE.md) file.
