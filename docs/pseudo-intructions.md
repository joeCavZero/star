# Pseudo-Instructions

Pseudo-instructions are higher-level assembly commands that make programming easier and more expressive. They are not directly supported by the virtual machine, but are expanded by the assembler into one or more native instructions before code generation.

---

## How Pseudo-Instructions Are Resolved

During the compilation, each pseudo-instruction is replaced by a sequence of native instructions that achieve the same effect. This is handled by a resolver stage. For example, the pseudo-instruction `nope` is replaced by `add $zero, $zero, $zero`, which does nothing.

Some pseudo-instructions expand into multiple instructions. The assembler may insert `nope` instructions after them to ensure correct label addressing and instruction alignment.

---

## Pseudo-Instructions Reference

### `nope`

No operation (does nothing).

```assembly
nope
```
**Resolves to:**
```assembly
add $zero, $zero, $zero
```

---

### `move`

Copies the value from one register to another.

```assembly
move $rd, $rs
```
**Resolves to:**
```assembly
add $rd, $zero, $rs
```

---

### `neg`

Negates a register (two's complement).

```assembly
neg $rd, $rs
```
**Resolves to:**
```assembly
sub $rd, $zero, $rs
```

---

### `jr`

Jumps to the address in a register.

```assembly
jr $rs
```
**Resolves to:**
```assembly
beqr $zero, $zero, $rs
```

---

### `ret`

Returns from a subroutine (jumps to `$ra`).

```assembly
ret
```
**Resolves to:**
```assembly
j $ra
```

---

### `li`

Loads a 16-bit immediate value into a register.

```assembly
li $rd, imm
```
**Resolves to:**
```assembly
lli $rd, imm<7...0>
lai $rd, imm<15...8>
```
*The immediate is split into bits <7...0> and <15...8>.*

---

### `la`

Loads the address of a label into a register.

```assembly
la $rd, label
```
**Resolves to:**
```assembly
lli $rd, label<7...0>
lai $rd, label<15...8>
```

---

### Arithmetic Pseudo-Instructions

#### `mul`

```assembly
mul $rd, $rs, $rt
```
**Resolves to:**
```assembly
mulhl $rs, $rt
add $rd, $zero, $low
```

#### `div`

```assembly
div $rd, $rs, $rt
```
**Resolves to:**
```assembly
divhl $rs, $rt
add $rd, $zero, $low
```

#### `mod`

```assembly
mod $rd, $rs, $rt
```
**Resolves to:**
```assembly
divhl $rs, $rt
add $rd, $zero, $high
```

---

### `swap`

Swaps the values of two registers.

```assembly
swap $r1, $r2
```
**Resolves to:**
```assembly
add $aux1, $zero, $r1
add $r1, $zero, $r2
add $r2, $zero, $aux1
```
*Uses `$aux1` as a temporary register.*

---

### Immediate Arithmetic Pseudo-Instructions

For each instruction below, replace `add` with the corresponding operation (`sub`, `and`, `or`, `xor`, `shl`, `shr`):

#### `addi`

```assembly
addi $rd, $rs, imm
```
**Resolves to:**
```assembly
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
add $rd, $rs, $aux1
```

#### `subi`, `andi`, `ori`, `xori`, `shli`, `shri`

Follow the same expansion as `addi`, replacing `add` with the appropriate operation.

---

### Increment/Decrement

#### `inc`

```assembly
inc $r
```
**Resolves to:**
```assembly
lli $aux1, 0x01
lai $aux1, 0x00
add $r, $r, $aux1
```

#### `dec`

```assembly
dec $r
```
**Resolves to:**
```assembly
lli $aux1, 0x01
lai $aux1, 0x00
sub $r, $r, $aux1
```

---

### Immediate Multiplication/Division/Modulo

#### `muli`

```assembly
muli $rd, $rs, imm
```
**Resolves to:**
```assembly
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
mulhl $rs, $aux1
add $rd, $zero, $low
```

#### `divi`

```assembly
divi $rd, $rs, imm
```
**Resolves to:**
```assembly
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
divhl $rs, $aux1
add $rd, $zero, $low
```

#### `modi`

```assembly
modi $rd, $rs, imm
```
**Resolves to:**
```assembly
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
divhl $rs, $aux1
add $rd, $zero, $high
```

---

### Conditional Branch Pseudo-Instructions

Each branch pseudo-instruction expands similarly, but is listed separately for clarity.

#### `beqa`

Branch if equal.

```assembly
beqa $rs, $rt, label
```
**Resolves to:**
```assembly
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
beqr $rs, $rt, $aux1
```
*Offset is computed as the relative distance to the label.*

#### `bneqa`

Branch if not equal.

```assembly
bneqa $rs, $rt, label
```
**Resolves to:**
```assembly
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
bneqr $rs, $rt, $aux1
```

#### `bgta`

Branch if greater than (signed).

```assembly
bgta $rs, $rt, label
```
**Resolves to:**
```assembly
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
bgtqr $rs, $rt, $aux1
```

#### `blta`

Branch if less than (signed).

```assembly
blta $rs, $rt, label
```
**Resolves to:**
```assembly
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
bltqr $rs, $rt, $aux1
```

#### `bgtua`

Branch if greater than (unsigned).

```assembly
bgtua $rs, $rt, label
```
**Resolves to:**
```assembly
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
bgtuqr $rs, $rt, $aux1
```

#### `bltua`

Branch if less than (unsigned).

```assembly
bltua $rs, $rt, label
```
**Resolves to:**
```assembly
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
bltuqr $rs, $rt, $aux1
```

---

### `ja`

Unconditional jump to a label.

```assembly
ja label
```
**Resolves to:**
```assembly
lli $aux1, label<7...0>
lai $aux1, label<15...8>
j $aux1
```

---

### Memory Access Pseudo-Instructions

Memory access pseudo-instructions expand into several instructions to compute addresses and perform loads/stores.

#### Example: `lw`

```assembly
lw $rd, label[offset]
```
**Resolves to:**
```assembly
lli $aux1, label<7...0>
lai $aux1, label<15...8>
lli $aux2, offset<7...0>
lai $aux2, offset<15...8>
add $aux1, $aux1, $aux2
lab $rd, $aux1
lli $aux2, 0x01
lai $aux2, 0x00
add $aux1, $aux1, $aux2
llb $rd, $aux1
```
*This sequence loads a 16-bit word from memory.*

---

## Note on Alignment

In the resolver phase, pseudo-instructions are primarily expanded into multiple `nope` instructions to ensure that the symbol table addresses are correctly aligned. This ensures that each label always points to the start of a real instruction, maintaining the integrity of jumps and branches.

You can read more about this in the [`resolver stage`](/docs/resolver-stage.md).

