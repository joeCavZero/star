# Instruction Formats

The **Star** instructions are grouped into several formats, each defining how operands are encoded and how the instruction is interpreted. Understanding these formats is essential for writing and reading Star assembly code.

| Format   | Structure Example                | Description                                                                                 |
|:--------:|:---------------------------------|:--------------------------------------------------------------------------------------------|
| Trinity  | `add $rd, $r1, $r2`              | Three registers: destination and two sources. Used for most arithmetic and logic operations. |
| Hime     | `lai $rd, imm8`                  | One register and an 8-bit immediate value. Used for loading immediates into registers.      |
| Pair     | `mulhl $r1, $r2`                 | Two registers. Used for operations like multiplication, division, and byte manipulation.    |
| Clover   | `j $rs`                          | Single register. Used for jump instructions.                                                |
| Ark      | `mcall`                          | No explicit operands; uses auxiliary registers for system/machine calls.                    |


## Details

- **Trinity**:  
  Used for instructions that operate on three registers.  
  Example: `add $rd, $r1, $r2` adds `$r1` and `$r2`, storing the result in `$rd`.

- **Hime**:  
  Used for instructions that load an 8-bit immediate value into a register.  
  Example: `lai $rd, imm8` loads the lower 8 bits of `imm8` into `$rd`.

- **Pair**:  
  Used for instructions that operate on two registers.  
  Example: `mulhl $r1, $r2` multiplies `$r1` and `$r2` (signed), result in `$low`/`$high`.

- **Clover**:  
  Used for instructions that operate on a single register.  
  Example: `j $rs` jumps to the address in `$rs`.

- **Ark**:  
  Used for machine/system calls. The operation and parameters are defined by the auxiliary registers.  
  Example: `mcall` performs a system call as specified by `$aux1`, `$aux2`, and `$aux3`.

## Codop Extension

The Codop extension is a technique used to increase the number of available instructions in an assembly language without modifying the underlying architecture. It achieves this by using specific encoding patterns to represent additional instructions, enabling more complex operations.

For example, if the opcode begins with a particular 4-bit sequence, such as `zzzz yyyy xxxx 0000`, the instruction is interpreted as a Trinity format, where `zzzz`, `yyyy`, and `xxxx` represent registers. If the opcode at the start, such as `0111` (for `lai`) or `1000` (for `lli`), is used, the instruction is interpreted as a Hime format.

The Hime format can be represented as `iiii iiii xxxx oooo`, where `iiii iiii` is the immediate value, `xxxx` is the destination register, and `oooo` is the operation code (e.g., `0111` or `1000`).

If the opcode is `1111`, the instruction is interpreted as a Pair format: `yyyy xxxx oooo 1111`, where `oooo` is the operation code and `xxxx` and `yyyy` are registers. This allows for new operation codes, such as `0000 1111` (for `mulhl`), but at the cost of reducing the number of bits available for other purposes.

The same logic applies to Clover and Ark formats. For Clover, the format is `xxxx oooo 1111 1111`, where `oooo` is the operation code and `xxxx` is the register. For Ark, the format is `oooo 1111 1111 1111`, where `oooo` is the operation code.

This extension method allows **Star** to support a wide range of instructions, even with a limited number of opcode bits.

> For a list of which instructions use each format, see the [instructions documentation](/docs/instructions.md).
