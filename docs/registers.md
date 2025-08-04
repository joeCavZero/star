# Registers

The origin of the registers in the Star Virtual Machine was inspired by classic computer architectures such as MIPS and RISC-V, which use a set of general-purpose registers to store temporary data and operation results. The choice of 16 registers of 16 bits is due to the limitations of the 16-bit architecture, providing enough registers for complex operations without overloading memory.

## General-Purpose Registers

The Star Virtual Machine has 16 general-purpose registers that store 16-bit values. They are used for temporary and static data during program execution and are directly accessible by assembly instructions.

### Register Numbering and Description

- **0 - Zero:** Always zero and cannot be modified.
- **1 - A:** Stores output for arithmetic and logical operations.
- **2 - B:** Holds temporary values during execution (source register).
- **3 - C:** Holds temporary values during execution (source register).
- **4 - D:** Destination register for arithmetic/logical instructions.
- **5 - E:** Used for temporary values (source register).
- **6 - F:** Destination register for arithmetic/logical instructions.
- **7 - G:** Used for temporary values (source register).
- **8 - Aux1:** Used by pseudo-instructions and as the primary interface in system calls.
- **9 - Aux2:** Used by pseudo-instructions and as the primary argument in system calls.
- **10 - Aux3:** Used by pseudo-instructions and as the secondary argument in system calls.
- **11 - Carry:** Stores the carry (or borrow) from arithmetic operations.
- **12 - Low:** Stores the lower part of results from operations that exceed 16 bits.
- **13 - High:** Stores the upper part of results from operations that exceed 16 bits.
- **14 - Return Address:** Stores the return address for jumps and branches.
- **15 - Stack Pointer:** Points to the top of the stack.

### The Zero Register

This register always holds the value zero. Any attempt to modify it is ignored.

### Auxiliary Registers

Auxiliary registers (Aux1, Aux2, and Aux3) are used for temporary storage and in pseudo-instructions/system calls. For example:

```python
addi $a, $a, 1
```

is translated to:

```python
lli $aux1, 0x01
lai $aux1, 0x00
add $a, $a, $aux1
```

### The Carry Register

The Carry register indicates a carry (or borrow) from arithmetic operations. Operations such as add, sub, shl, and shr affect it. For example:

```python
li $a, 0xFFFF
add $b, $a, $a
```

After execution:
- $a = 0xFFFF
- $b = 0xFFFE
- $carry = 0x0001

### The Low and High Registers

These registers store parts of the result when an operation produces a value larger than 16 bits. In multiplication, for example, the Low register gets the lower part and the High register the upper part.

Example:

```python
li $a, 0x0002
li $b, 0x0003
mulhl $a, $b
```

Results:
- $a = 0x0002
- $b = 0x0003
- $low = 0x0006
- $high = 0x0000

### The Return Address Register

This register stores the return address for subroutine calls. Its value is automatically updated as:

    Return Address = Current Program Counter + 1

Example usage:

```python
ja start
func: 
    mul $a, $b, $C
    j $ra
start:
    ja func
    addi $a, $a, 2
```

### Hidden Registers

The Star Virtual Machine also uses hidden registers that are managed internally:
- **Program Counter:** A 16-bit register storing the index of the next instruction.
- **Instruction Pointer Register:** Stores the real address of the instruction. Its computation is:
  - High part: Program Counter × 2
  - Low part: Program Counter × 2 + 1
- **Instruction Register:** A 16-bit register holding the current instruction.

