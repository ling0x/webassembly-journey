# WebAssembly Binary Toolkit (wabt or wabbit)

https://github.com/WebAssembly/wabt

clone and build wabt according to its README

add the build folder to path so you have the following utilities:

## wat2wasm

While not technically a compiler, it can help the learning process to think of
it that way. This tool takes raw wat code (which we’ll be writing in this
chapter) and produces binary WebAssembly modules from it.

## wasm2wat

The opposite of wat2wasm, this tool “decompiles” a binary WebAssembly module and
produces a wat text file.

## wasm-objdump

An extremely useful tool for printing and querying information about the
contents of a wasm binary file.

## wasm-interp

This tool reads and interprets wasm binaries, allowing you to invoke functions
in the module.

## wasm-strip

This tool is used to reduce the size of wasm binaries by stripping out various
sections and components.
