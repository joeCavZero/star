# Processors

Processors are special directives that allow the assembler to perform advanced behaviors, such as including files and defining macros.

In the **Star Virtual Machine** there are two main types of processors:
- **Include**: Allows the inclusion of external files into the source code.
- **Define**: Allows the definition of macros that can be used throughout the code.

You can read more about how processors are handled in the [scanner stage documentation](/docs/scanner-stage.md).

## Include Processor

The include processor allows you to include the contents of another file into the current source code. This is useful for modularizing code and reusing common definitions.

```python
@include "file.asm"
```

## Define Processor

The define processor allows you to define a macro that can be used throughout the code. Macros are placeholders that can be replaced with specific values or code snippets during the assembly process.

```python
@define MY_MACRO 42
.instr
    addi $a, $b, MY_MACRO # here MY_MACRO will be replaced with 42
```

Star macros can also accept parameters, allowing for more flexible code generation.

```python
@define ADD_TWO_NUMBERS(%a, %b) \
    addi $aux1, %a, %b
.instr
    ADD_TWO_NUMBERS($a, $b) # here %a and %b will be replaced with the values of $a and $b
```

## Once Processor
The once processor allows you to include a file only once, preventing multiple inclusions of the same file. This is useful for avoiding redefinitions and ensuring that the code is only processed once.

```python
@once
@define MY_MACRO 42
@define SUM_MACRO(%a, %b) \
    addi $aux1, %a, %b
```