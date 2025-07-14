# Directives

Directives are special tokens in the source code that provide instructions to the assembler. They are not part of the executable code but guide the assembly process. In the **Star Virtual Machine**, directives are used to define bytes, words, spaces, strings, data sections, and instruction sections.

You can read more about how directives are processed in the [parser stage](/docs/parser-stage.md) and [generation stage](/docs/generation-stage.md).

## Types of Directives

### `.byte`
Defines a byte (8 bits) in the data section. It can be used to initialize memory with specific values.

```python
.byte 200, 0b0001100, 0x7F
```

### `.word`
Defines a word (16 bits) in the data section. It can be used to initialize memory with specific values.

```python
.word 0x1234, 0b1010101010101010, 30000
```

### `.space`
Allocates a specified number of bytes in the data section without initializing them.

```python
.space 10
```

### `.string`
Defines a string in the data section. It does not automatically append a zero byte at the end, so it is not null-terminated.

```python
.string "Hello, World!"
```

### `.stringz`
Defines a null-terminated string in the data section. It automatically appends a zero byte at the end of the string.

```python
.stringz "Hello, World!"
```

### `.checkpoint`
Defines a checkpoint in the data section. It can be used to mark a specific point in the data section for debugging or reference purposes. It does not accept any arguments.

```python
.checkpoint
```

### `.data`
Marks the start of a data section. All subsequent `.byte`, `.word`, `.space`, `.string`, and `.stringz` directives will be placed in this section.

```python
.data
    label_1: .byte 100
    label_2: .word 0b011
    label_3: .string "Hello"
    label_4: .stringz "World!"
    label_5: .space 5
```

### `.instr`
Marks the start of an instruction section. All subsequent instructions will be placed in this section.

```python
.instr
    addi $a, $b, 10
    sub $c, $d, $e
```

> For more information about how labels and symbols are handled, see the [symbol table documentation](/docs/symbol-table.md).