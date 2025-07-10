# Processors

Processors are special directives that allow the assembler to perform advanced behaviors, such as including files and defining macros.

In the **Star Virtual Machine** there are two main types of processors:
- **Include**: Allows the inclusion of external files into the source code.
- **Define**: Allows the definition of macros that can be used throughout the code.

You can read more about how processors are handled in the [scanner stage documentation](/docs/scanner-stage.md).

## Include Processor

The include processor allows you to include the contents of another file into the current source code. This is useful for modularizing code and reusing common definitions.

```assembly
@include "file.asm"
```

## Define Processor

The define processor allows you to define a macro that can be used throughout the code. Macros are placeholders that can be replaced with specific values or code snippets during the assembly process.

```assembly
@define MY_MACRO 42
.instr
    addi $a, $b, MY_MACRO # here MY_MACRO will be replaced with 42
```