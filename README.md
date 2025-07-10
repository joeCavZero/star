<div align="center">
  <img src="assets/star-logo.png" width="200" />
</div>

<h1 align="center">STAR VIRTUAL MACHINE</h1>

A 16-bit virtual machine and programming language designed for educational purposes.

---
## A Simple "Hello, World!"

```asm
.data
    string: .stringz "Hello World!"
.instr
start:
        la $g, string
        li $a, 0
loop:   
        llb $a, $g
        beqa $a, $zero, end
        
        li $aux1, 7
        move $aux2, $a
        mcall

        inc $g
        ja loop
end:    nope
```

---

## Usage
### Displaying the Version
To display the version of the **Star**, use the following command:

```bash
star --version
```

### Running a Program
To run a program file, use the following command:

```bash
star file.asm
```
---
## Documentation
### Syntax
- [Instructions](/docs/instructions.md): Learn about the available instructions in the STAR assembly language.
- [Pseudo-Instructions](/docs/pseudo-intructions.md): Learn about pseudo-instructions and how they simplify assembly programming.
- [Directives](/docs/directives.md): Understand the directives used in STAR assembly language
- [Processors](/docs/processors.md): Explore the processors that enhance the assembly language capabilities.
- [Machine Calls](/docs/machine-calls.md): Learn about the special instructions for system interaction and I/O operations.

### Memory
- [Instruction Memory](/docs/instruction-memory.md): Understand how instruction memory works and its role in the virtual machine.
- [Data Memory](/docs/data-memory.md): Learn about data memory and how it is used to store values.
- [Position Memory](/docs/position-memory.md): Learn about position memory and its importance for debugging and error reporting.

### Stages
- [Scanner Stage](/docs/scanner-stage.md): Understand how the scanner reads source code and converts it into tokens.
- [Parser Stage](/docs/parser-stage.md): Learn how the parser analyzes tokens and constructs an abstract syntax tree (AST).
- [Resolver Stage](/docs/resolver-stage.md): Discover how the resolver processes the AST and resolves symbols.
- [Generation Stage](/docs/generation-stage.md): Understand how the assembler generates machine code
- [Execution Stage](/docs/execution-stage.md): Learn how the virtual machine executes the generated code.