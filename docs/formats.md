# Instruction Formats

The Star VM instructions are grouped into several formats, each defining how operands are encoded and how the instruction is interpreted. Understanding these formats is essential for writing and reading Star assembly code.

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

> For a list of which instructions use each format, see the [instructions documentation](/docs/instructions.md).
