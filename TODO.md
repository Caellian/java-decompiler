# TODO List

## Crucial

- Generate java files from class structure
- Handle invalid names (which cause compile errors)
- Handle nested classes
- Remove synthetic class fields & methods
- Store output into input relative paths or into -o argument

## Big changes

- Deobfuscation support
- Separate class handling code into it's own library
- Allow running the decompiler as a daemon
    - Useful for environments where jars are being decompiled partially (like IDEs)
- Multithreading
- Ability to modify JARs
    - How do we tell the decompiler what to do?
    - Maybe a separate program?
- Support for other JVM languages (primarily Kotlin)

## Child projects

- Binary class diff format for compact patches
    - Minimizes copyright infringement caused by patches containing program data
    - Less likely to break between versions of patched software if done properly
- JNI bindings
    - Probably required for full Gradle integration

## Niceties

- Provide option to preview pre-decompiled class structure (like javap)
- Allow piping decompiled classes into stdout
- Support for proguard generated mappings?

## Project

- Add CPack configuration.
    - Create a GitHub  action for creating packages on tagged commits.
        - https://help.github.com/en/actions/reference/workflow-syntax-for-github-actions#onpushpull_requestbranchestags
        - https://github.com/AButler/upload-release-assets
        - https://stackoverflow.com/questions/18216991/create-a-tag-in-a-github-repository
        - https://github.com/anothrNick/github-tag-action
