# Data Memory

Data memory is a crucial part of the **Star Virtual Machine**. It stores bytes that can be accessed by the program during execution.

It uses a Big Endian layout, meaning that the most significant byte is stored at the lowest address. This affects how data is read and written in memory.

## Memory Structure

Data memory is organized as a linear array of bytes. Each byte can be accessed by its address using registers. The memory is divided into two main sections: the data section and the instruction section.

You can read more about how data is defined in the [directives documentation](/docs/directives.md) and how it is accessed in the [instructions documentation](/docs/instructions.md).

## Example of Data Memory Usage

```python
.data
    .byte 2
    .word 4
    .stringz "Hi"
```
The code above produces the following data memory layout:

| Address | Value |
|:-------:|:-----:|
| 0x0000  | 0x02  |
| 0x0001  | 0x00  |
| 0x0002  | 0x04  |
| 0x0003  | 'H'   |
| 0x0004  | 'i'   |
| 0x0005  | 0x00  |

> For more on how data memory is used during execution, see the [execution stage documentation](/docs/execution-stage.md).
