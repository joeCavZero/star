# Generation Stage

The generation stage is responsible for converting the Abstract Syntax Tree (AST) produced by the parser and resolved by the resolver into a sequence of native instructions in the instruction memory and data in the data memory that can be executed by the virtual machine. This stage is where the actual code generation happens, transforming high-level constructs into low-level instructions.

## How the Generation Stage Works

The generation stage iterates through the AST and generates the corresponding native instructions for each item in the instruction section. It also handles the data section, generating the necessary data items based on the definitions found in the AST.

You can read more about instruction formats in the [formats documentation](/docs/formats.md) and about the instruction set in the [instructions documentation](/docs/instructions.md).

## Example of Generation Output
```python
.instr
    nope
```
The generation stage would produce the following output:
```python
00000000 00000000
```
