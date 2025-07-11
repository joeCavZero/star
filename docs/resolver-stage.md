# Resolver Stage

The resolver stage is responsible for generating the symbol table and resolving pseudo-instructions from the Abstract Syntax Tree (AST) produced by the parser. This stage ensures that all labels and symbols are correctly defined and that pseudo-instructions are expanded into their corresponding native instructions.

## How the Resolver Works

The resolver stage can be divided into three main processes:
1. **Resolve Space**: This process adds multiple `nope` instructions after pseudo-instructions to ensure that the symbol table addresses are correctly aligned. This ensures that each label always points to the start of a real instruction.
2. **Resolve Symbol Table**: This process resolves the symbol table in the data and instruction sections of the AST. It ensures that all labels and symbols are defined and that their addresses are correctly calculated.
3. **Resolve Instructions**: This process resolves the pseudo-instructions in the instruction section of the AST. It expands pseudo-instructions into their corresponding native instructions.

You can read more about pseudo-instructions and how they are resolved in the [pseudo-instructions documentation](/docs/pseudo-intructions.md).

## Example of a Resolver Output
```python
.data
    label_1: .byte 2
.instr
    li $a, 10
```
The resolver would produce the following output:
```python
.data
    label_1: .byte 2
.instr
    lli $a, 10<7...0>
    lai $a, 10<15...8>
```
```
Symbol Table:
    - label_1: 0x0000
```