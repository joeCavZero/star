# Symbol Table

The symbol table is a crucial component of the **Star Virtual Machine**. It manages the addresses of labels and symbols in the code, ensuring that they are correctly defined and resolved during the assembly process.

It maps labels to their corresponding addresses in the data and instruction sections.

Labels in the instruction section are mapped to addresses, also called the program counter (PC), of the instructions they represent. Labels in the data section are mapped to the addresses of the data they define.

You can read more about how the symbol table is built in the [resolver stage documentation](/docs/resolver-stage.md).

## Example of a Symbol Table

```python
.data
    label_1: .byte 2
    label_2: .word 4
.instr
    label_3: addi $a, $b, 10
```
The symbol table for the above code would look like this:

```plaintext
Symbol Table:
    - label_1: 0
    - label_2: 1
    - label_3: 0
```