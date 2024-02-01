# Surtur

- [Introduction](#introduction)

- [Installation](#installation)

- [Documentation](#documentation)

- [Contributors](#contributors)

## Introduction

A build tool for c

This project is inspired by the WIP C build tool [loki](https://github.com/hllhnd/loki) but with an approach that is not exactly like cargo.

Under the hood this uses [gcc](https://gcc.gnu.org/) to compile and link libraries

## Installation

Right now you have to compile this manually with rust

## Getting started as a dev

Currently only linux is supported (macOS might work as well). Windows support would be a cool contribution :D

Requirements:

- GCC
- Rust

Clone the project using: `git clone https://github.com/Thepigcat76/surtur.git`

Run `cargo run` to get an introduction to all commands.

Create a project with `cargo run new my-project`

Navigate to the directory using `cd my-project`

Run the project using `cargo run run`

Run `cargo run dbg-deps` to install some dependencies (only for debugging)

## Todo

- Windows and macOS support (adjust commands and c-compiler)
- Support for "nested" projects (having c files and their header files in another directory in the src directory)
- Testing system for testing your C code (environment variable controlled test execution?)
- support for multiple compilers
- dependency linking

## Documentation

Coming soon!

## Contributors

- [Thepigcat76](https://github.com/Thepigcat76)
