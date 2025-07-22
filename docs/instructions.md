# Star Instructions

This table presents all native instructions of the **Star Virtual Machine**, including their formats, descriptions, and syntax. 


| Instruction    | Format    | Syntax                                   | Description                                                                                   |
|:-------------- |:---------:|:-----------------------------------------|:----------------------------------------------------------------------------------------------|
| **add**        | Trinity   | `add $rd, $r1, $r2`                      | Adds the values in `$r1` and `$r2`, storing the result in `$rd`.                              |
| **sub**        | Trinity   | `sub $rd, $r1, $r2`                      | Subtracts the value in `$r2` from `$r1`, storing the result in `$rd`.                         |
| **and**        | Trinity   | `and $rd, $r1, $r2`                      | Performs bitwise AND between `$r1` and `$r2`, storing the result in `$rd`.                    |
| **or**         | Trinity   | `or $rd, $r1, $r2`                       | Performs bitwise OR between `$r1` and `$r2`, storing the result in `$rd`.                     |
| **xor**        | Trinity   | `xor $rd, $r1, $r2`                      | Performs bitwise XOR between `$r1` and `$r2`, storing the result in `$rd`.                    |
| **shl**        | Trinity   | `shl $rd, $r1, $r2`                      | Shifts the value in `$r1` left by the amount in `$r2`, storing the result in `$rd`.           |
| **shr**        | Trinity   | `shr $rd, $r1, $r2`                      | Shifts the value in `$r1` right by the amount in `$r2`, storing the result in `$rd`.          |
| **lai**        | Hime      | `lai $rd, imm8`                          | Loads the lower 8 bits of the immediate value into `$rd` (upper 8 bits remain unchanged).      |
| **lli**        | Hime      | `lli $rd, imm8`                          | Loads the upper 8 bits of the immediate value into `$rd` (lower 8 bits remain unchanged).      |
| **beqr**       | Trinity   | `beqr $r1, $r2, $rt`                     | If `$r1` equals `$r2`, jumps forward by `$rt` instructions.                                   |
| **bneqr**      | Trinity   | `bneqr $r1, $r2, $rt`                    | If `$r1` does not equal `$r2`, jumps forward by `$rt` instructions.                           |
| **bgtr**       | Trinity   | `bgtr $r1, $r2, $rt`                     | If `$r1` is greater than `$r2` (signed), jumps forward by `$rt` instructions.                 |
| **bltr**       | Trinity   | `bltr $r1, $r2, $rt`                     | If `$r1` is less than `$r2` (signed), jumps forward by `$rt` instructions.                    |
| **bgtur**      | Trinity   | `bgtur $r1, $r2, $rt`                    | If `$r1` is greater than `$r2` (unsigned), jumps forward by `$rt` instructions.               |
| **bltur**      | Trinity   | `bltur $r1, $r2, $rt`                    | If `$r1` is less than `$r2` (unsigned), jumps forward by `$rt` instructions.                  |
| **mulhl**      | Pair      | `mulhl $r1, $r2`                         | Multiplies `$r1` and `$r2` (signed); result is split between `$low` and `$high`.              |
| **divhl**      | Pair      | `divhl $r1, $r2`                         | Divides `$r1` by `$r2` (signed); quotient in `$low`, remainder in `$high`.                    |
| **muluhl**     | Pair      | `muluhl $r1, $r2`                        | Multiplies `$r1` and `$r2` (unsigned); result is split between `$low` and `$high`.            |
| **divuhl**     | Pair      | `divuhl $r1, $r2`                        | Divides `$r1` by `$r2` (unsigned); quotient in `$low`, remainder in `$high`.                  |
| **not**        | Pair      | `not $rd, $rs`                           | Performs bitwise NOT on `$rs`, storing the result in `$rd`.                                   |
| **xlb**         | Pair      | `xlb $rd, $rs`                            | Sign-extends the lower byte of `$rs` to 16 bits and stores the result in `$rd`.               |
| **lab**        | Pair      | `lab $rd, $rs`                           | Loads the high byte from memory at the address in `$rs` into `$rd`.                           |
| **llb**        | Pair      | `llb $rd, $rs`                           | Loads the low byte from memory at the address in `$rs` into `$rd`.                            |
| **sab**        | Pair      | `sab $rs, $rd`                           | Stores the high byte of `$rs` into memory at the address in `$rd`.                            |
| **slb**        | Pair      | `slb $rs, $rd`                           | Stores the low byte of `$rs` into memory at the address in `$rd`.                             |
| **j**          | Clover    | `j $rs`                                  | Jumps to the address contained in `$rs`.                                                      |
| **mcall**      | Ark       | `mcall`                                  | Calls a machine routine (system call); operation is defined by `$aux1`.                       |

The **Star** assembles these instructions into a binary format that can executed. Each instruction is encoded based on its format, which determines how operands are represented and how the instruction is interpreted.

## Binary Formats

Binary formats are the specific encoding of instructions in the **Star**. Each instruction is represented by a 16-bit binary code, which can be broken down into fields that represent the operation and its operands. The following table shows the binary structure of each instruction: 

| Instruction    | 15...12 | 11...8 | 7...4 | 3...0 |
|:-------------- |:-------:|:------:|:-----:|:-----:|
| **add**        | zzzz    | yyyy   | xxxx  | 0000  |
| **sub**        | zzzz    | yyyy   | xxxx  | 0001  |
| **and**        | zzzz    | yyyy   | xxxx  | 0010  |
| **or**         | zzzz    | yyyy   | xxxx  | 0011  |
| **xor**        | zzzz    | yyyy   | xxxx  | 0100  |
| **shl**        | zzzz    | yyyy   | xxxx  | 0101  |
| **shr**        | zzzz    | yyyy   | xxxx  | 0110  |
| **lai**        | iiii    | iiii   | xxxx  | 0111  |
| **lli**        | iiii    | iiii   | xxxx  | 1000  |
| **beqr**       | zzzz    | yyyy   | xxxx  | 1001  |
| **bneqr**      | zzzz    | yyyy   | xxxx  | 1010  |
| **bgtr**       | zzzz    | yyyy   | xxxx  | 1011  |
| **bltr**       | zzzz    | yyyy   | xxxx  | 1100  |
| **bgtur**      | zzzz    | yyyy   | xxxx  | 1101  |
| **bltur**      | zzzz    | yyyy   | xxxx  | 1110  |
| **mulhl**      | yyyy    | xxxx   | 0000  | 1111  |
| **divhl**      | yyyy    | xxxx   | 0001  | 1111  |
| **muluhl**     | yyyy    | xxxx   | 0010  | 1111  |
| **divuhl**     | yyyy    | xxxx   | 0011  | 1111  |
| **not**        | yyyy    | xxxx   | 0100  | 1111  |
| **xlb**        | yyyy    | xxxx   | 0101  | 1111  |
| **lab**        | yyyy    | xxxx   | 0110  | 1111  |
| **llb**        | yyyy    | xxxx   | 0111  | 1111  |
| **sab**        | yyyy    | xxxx   | 1000  | 1111  |
| **slb**        | yyyy    | xxxx   | 1001  | 1111  |
| **j**          | xxxx    | 0000   | 1111  | 1111  |
| **mcall**      | 0000    | 1111   | 1111  | 1111  |

In this table `xxxx`, `yyyy`, and `zzzz` represent registers, while `iiii` represents immediate values. The last four bits (`0000`, `0001`, etc.) represent the specific operation code for each instruction.

Formats categorize instructions by their structure and usage. See the [formats documentation](/docs/formats.md) for more details.

Pseudo-instructions can be found in the [pseudo-instructions documentation](/docs/pseudo-intructions.md).