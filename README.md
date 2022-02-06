
# varsion.rs

varsion is a rust program that attempts to handle source code versioning universally by placing a VERSION file at the root of the source code. it uses this VERSION file to handle Semantic Versioning for the developers. the user just provides what segment of the version to change and it will handle everything else. varsion allows for pre-release and build segments to be added to the version.

## download

the only way to download the executable is to download the source code. the compiled binaries can be seen in the .bin folder where each version will be listed along with a zip of it. the binaries are the compiled program from my Windows machine using arm64. other platform binaries will be released later on you may also compile the source code yourself using the rust compiler. a real download method will be developed and the binaries will be removed.

## compile

to compile the source code one must have the rust compiler and the cargo package manager installed. compile like a normal rust program.

## wheres version 1?

version 1 was in go, version 2 is in rust.

version 1 of this project was done in go and had a lot of issues with how it was implemented.
as new issues arose and new features were being added, the shortcomings of the code began to show.
the binaries and the source code can be found on github.

<https://github.com/chrwalte/varsion>

## initial project plans

automated-version autoversion command line interface tool that will attempt to keep track of the global project version user must provide what version segment to increase

project version stored in VERSION file VERSION file can be used for anything to do with version

version type examples: version major version minor version patch will increase the given segment by 1 making changes to the others as needed

Semantic Versioning 2.0.0 semver.org
