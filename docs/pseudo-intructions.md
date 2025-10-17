# Pseudo-Instructions

Pseudo-instructions are higher-level assembly commands that make programming easier and more expressive. They are not directly supported by the virtual machine, but are expanded by the assembler into one or more native instructions before code generation.

---

## How Pseudo-Instructions Are Resolved

During compilation, each pseudo-instruction is replaced by a sequence of native instructions that achieve the same effect. This is handled by a resolver stage. For example, the pseudo-instruction `nope` is replaced by `add $zero, $zero, $zero`, which performs no operation.

Some pseudo-instructions expand into multiple instructions. The assembler may insert `nope` instructions after them to ensure correct label addressing and instruction alignment.

---

## Pseudo-Instructions Reference

### `nope`

Performs no operation.

```python
nope
```
**Expands to:**
```python
add $zero, $zero, $zero
```

---

### `move`

Copies the value from one register to another.

```python
move $rd, $rs
```
**Expands to:**
```python
add $rd, $zero, $rs
```

---

---

### `jr`

Jumps to the address contained in a register.

```python
jr $rs
```
**Expands to:**
```python
beqr $zero, $zero, $rs
```

---

### `ret`

Returns from a subroutine by jumping to `$ra`.

```python
ret
```
**Expands to:**
```python
j $ra
```

---

### `li`

Loads a 16-bit immediate value into a register.

```python
li $rd, imm
```
**Expands to:**
```python
lli $rd, imm<7...0>
lai $rd, imm<15...8>
```
*The immediate is split into bits <7...0> and <15...8>.*

---

### `la`

Loads the address of a label into a register.

```python
la $rd, label
```
**Expands to:**
```python
lli $rd, label<7...0>
lai $rd, label<15...8>
```

---

### `lb`

Loads a byte from memory and sign-extends it to a word.

```python
lb $rd, $rs[offset]
```
**Expands to:**
```python
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
add $aux1, $aux1, $rs
llb $rd, $aux1
xlb $rd, $rd
```

---

### `sb`

Stores the least significant byte of a register into memory at a computed address.

```python
sb $rd, $rs[offset]
```
**Expands to:**  
Let `address` be *`label_address + offset`*:
```python
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
add $aux1, $aux1, $rs
slb $rd, $aux1
```

---

### `lw`

Loads a 16-bit word from memory with a offset.

```python
lw $rd, $rs[offset]
```
**Expands to:**  
```python
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
add $aux1, $aux1, $rs
lab $rd, $aux1
lli $aux2, 0x01
lai $aux2, 0x00
add $aux1, $aux1, $aux2
llb $rd, $aux1
```

---

### `sw`

Stores a 16-bit word from a register into memory at a computed address, saving the alternate and low bytes separately.

```python
sw $rd, $rs[offset]
```
**Expands to:**  
```python
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
add $aux1, $aux1, $rs
sab $rd, $aux1
lli $aux2, 0x01
lai $aux2, 0x00
add $aux1, $aux1, $aux2
slb $rd, $aux1
```

---

### `mul`

Multiplies two registers and stores the result in a register.

```python
mul $rd, $rs, $rt
```
**Expands to:**
```python
mulhl $rs, $rt
add $rd, $zero, $low
```

---

### `div`

Divides one register by another and stores the quotient in a register.

```python
div $rd, $rs, $rt
```
**Expands to:**
```python
divhl $rs, $rt
add $rd, $zero, $low
```

---

### `mod`

Divides one register by another and stores the remainder in a register.

```python
mod $rd, $rs, $rt
```
**Expands to:**
```python
divhl $rs, $rt
add $rd, $zero, $high
```

---

### `swap`

Swaps the values of two registers using a temporary register.

```python
swap $r1, $r2
```
**Expands to:**
```python
add $aux1, $zero, $r1
add $r1, $zero, $r2
add $r2, $zero, $aux1
```

---

### `addi`

Adds an immediate value to a register.

```python
addi $rd, $rs, imm
```
**Expands to:**
```python
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
add $rd, $rs, $aux1
```

---

### `subi`

Subtracts an immediate value from a register.

```python
subi $rd, $rs, imm
```
**Expands to:**
```python
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
sub $rd, $rs, $aux1
```

---

### `andi`

Performs a bitwise AND operation between a register and an immediate value.

```python
andi $rd, $rs, imm
```
**Expands to:**
```python
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
and $rd, $rs, $aux1
```

---

### `ori`

Performs a bitwise OR operation between a register and an immediate value.

```python
ori $rd, $rs, imm
```
**Expands to:**
```python
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
or $rd, $rs, $aux1
```

---

### `xori`

Performs a bitwise XOR operation between a register and an immediate value.

```python
xori $rd, $rs, imm
```
**Expands to:**
```python
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
xor $rd, $rs, $aux1
```

---

### `shli`

Shifts the bits of a register to the left by an immediate value.

```python
shli $rd, $rs, imm
```
**Expands to:**
```python
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
shl $rd, $rs, $aux1
```

---

### `shri`

Shifts the bits of a register to the right by an immediate value.

```python
shri $rd, $rs, imm
```
**Expands to:**
```python
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
shr $rd, $rs, $aux1
```

---

### `inc`

Increments the value in a register by 1.

```python
inc $r
```
**Expands to:**
```python
lli $aux1, 0x01
lai $aux1, 0x00
add $r, $r, $aux1
```

---

### `dec`

Decrements the value in a register by 1.

```python
dec $r
```
**Expands to:**
```python
lli $aux1, 0x01
lai $aux1, 0x00
sub $r, $r, $aux1
```

---

### `muli`

Multiplies a register by an immediate value.

```python
muli $rd, $rs, imm
```
**Expands to:**
```python
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
mulhl $rs, $aux1
add $rd, $zero, $low
```

---

### `divi`

Divides a register by an immediate value and stores the quotient.

```python
divi $rd, $rs, imm
```
**Expands to:**
```python
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
divhl $rs, $aux1
add $rd, $zero, $low
```

---

### `modi`

Divides a register by an immediate value and stores the remainder.

```python
modi $rd, $rs, imm
```
**Expands to:**
```python
lli $aux1, imm<7...0>
lai $aux1, imm<15...8>
divhl $rs, $aux1
add $rd, $zero, $high
```

---

### `beqa`

Branches if two registers are equal.

```python
beqa $rs, $rt, label
```
**Expands to:**
```python
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
beqr $rs, $rt, $aux1
```
*Offset is computed as the relative distance to the label.*

---

### `bneqa`

Branches if two registers are not equal.

```python
bneqa $rs, $rt, label
```
**Expands to:**
```python
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
bneqr $rs, $rt, $aux1
```

---

### `bgta`

Branches if one register is greater than another (signed).

```python
bgta $rs, $rt, label
```
**Expands to:**
```python
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
bgtqr $rs, $rt, $aux1
```

---

### `blta`

Branches if one register is less than another (signed).

```python
blta $rs, $rt, label
```
**Expands to:**
```python
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
bltqr $rs, $rt, $aux1
```

---

### `bgtua`

Branches if one register is greater than another (unsigned).

```python
bgtua $rs, $rt, label
```
**Expands to:**
```python
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
bgtuqr $rs, $rt, $aux1
```

---

### `bltua`

Branches if one register is less than another (unsigned).

```python
bltua $rs, $rt, label
```
**Expands to:**
```python
lli $aux1, offset<7...0>
lai $aux1, offset<15...8>
bltuqr $rs, $rt, $aux1
```

---

### `ja`

Performs an unconditional jump to a label.

```python
ja label
```
**Expands to:**
```python
lli $aux1, label<7...0>
lai $aux1, label<15...8>
j $aux1
```

---

## Note on Alignment

In the resolver phase, pseudo-instructions are primarily expanded into multiple `nope` instructions to ensure that the symbol table addresses are correctly aligned. This ensures that each label always points to the start of a real instruction, maintaining the integrity of jumps and branches.

You can read more about this in the [`resolver stage`](/docs/resolver-stage.md).

