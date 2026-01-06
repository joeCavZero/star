<div align="center">
  <img src="/docs/images/star-logo.png" width="300" />
</div>

<h1 align="center">STAR VIRTUAL MACHINE</h1>

A 16-bit virtual machine and assembly programming language designed for educational purposes.

---
## A Simple "Hello, World!"

```python
.data
    string: .stringz "Hello, World!" # defines a string in memory
.instr
start:
        la $g, string # load string address into $g
        li $a, 0 # load 0 into $a
loop:   
        llb $a, $g # load $a with the byte at address $g
        beqa $a, $zero, end # if $a is 0, jump to end
        
        li $aux1, 7 # load 7 into $aux1
        move $aux2, $a # move $a to $aux2
        mcall # syscall

        inc $g # increments $g by 1
        ja loop # jump to loop
end:    nope # no operation (end of program)
```

The **Star Virtual Machine** assembly language is designed to be simple and educational, allowing users to learn the basics of assembly programming and low-level concepts. 

The above program demonstrates how to print "Hello, World!" by loading a string from memory and using a machine call to output each character.

---

## Introduction
To get started with the **Star Virtual Machine** you can read the the [introduction](/docs/introduction.md).

---
## Documentation
### Syntax
To learn about the syntax of the **Star**
you can read the following documentation:
- [Instructions](/docs/instructions.md): Learn about the available instructions in the **Star**.
- [Pseudo-Instructions](/docs/pseudo-intructions.md): Learn about pseudo-instructions and how they simplify assembly programming.
- [Registers](/docs/registers.md): Understand the registers used in the **Star** and their purposes.
- [Directives](/docs/directives.md): Understand the directives used in **Star**.
- [Processors](/docs/processors.md): Explore the processors that enhance the assembly language capabilities.
- [Machine Calls](/docs/machine-calls.md): Learn about system interaction and I/O operations.

### Memory
To understand the memory model of the **Star**
you can read the following documentation:
- [Instruction Memory](/docs/instruction-memory.md): Understand how instruction memory works and its role in the virtual machine.
- [Data Memory](/docs/data-memory.md): Learn about data memory and how it is used to store values.
- [Position Memory](/docs/position-memory.md): Learn about position memory and its importance for debugging and error reporting.

### Compilation Stages
To learn about how the **Star** compilation processes you can read the following documentation:
- [Scanner Stage](/docs/scanner-stage.md): Understand how the scanner reads source code and converts it into tokens.
- [Parser Stage](/docs/parser-stage.md): Learn how the parser analyzes tokens and constructs an abstract syntax tree (AST).
- [Resolver Stage](/docs/resolver-stage.md): Discover how the resolver processes the AST and resolves symbols.
- [Generation Stage](/docs/generation-stage.md): Understand how the assembler generates machine code
- [Execution Stage](/docs/execution-stage.md): Learn how the virtual machine executes the generated code.