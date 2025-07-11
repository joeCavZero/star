<div align="center">
  <img src="/assets/star-logo.png" width="300" />
</div>

<h1 align="center">STAR VIRTUAL MACHINE</h1>

# Introduction
The **Star Virtual Machine** is a 16-bit virtual machine and programming language designed for educational purposes. It provides a simple yet powerful assembly language that allows users to learn the fundamentals of computer architecture, assembly programming, and low-level system interaction.

The **Star** is designed to be easy to understand and use, making it an ideal tool for students and beginners in computer science. It features a small set of instructions, a straightforward memory model, and a focus on basic operations that are essential for understanding how computers work.

This project includes a collection of example assembly programs located in the [examples](/examples/) folder. These examples are designed to help you learn and practice assembly programming with the Star Virtual Machine. The programs demonstrate a variety of concepts, ranging from simple arithmetic operations to more advanced algorithms and data structures. Exploring and modifying these examples is a great way to deepen your understanding of assembly language and the Star VM architecture.

## Installation
To install the **Star Virtual Machine**, you need to build it from source.

### Build Instructions

1. **If you're using Windows, ensure that you have the Visual C++ Build Tools installed**:
    - You'll need to install the [Visual C++ Build Tools](https://visualstudio.microsoft.com/pt-br/visual-cpp-build-tools/).
    - Select the "Desktop development with C++" workload during installation.
    - Press the "Install" button to install the required components.
    - This is a rust requirement for building the project on Windows.
    - If you're using Linux or MacOS, you can skip this step.
2. **Ensure that you have Rust and Cargo installed**:
    - Follow the installation guide on the official [Rust website](https://www.rust-lang.org/learn/get-started) if needed.
   

3. **Clone the repository**:
    - Use the following command to clone the repository:
      ```bash
      git clone https://github.com/joeCavZero/star.git
      ```
    - Alternatively, you can download the repository as a ZIP file and extract it.
  
4. **Build the source code**:
   - Run the following command to build the project in release mode:
     ```bash
     cargo build --release
     ```

5. **Locate the generated binary**:
   - After the build is complete, the binary will be located at:
     ```bash
     ./target/release/star.exe on Windows 
     ( or star.app on MacOS or star on Linux )
     ```

6. **Copy the binary to a convenient location**:
   - Copy the binary to a location where you can easily access it, such as a directory in your system's PATH.
   - You can also save the binary in the environment variable PATH, allowing you to run the program from any directory.

---

## The Command Line Interface
The **Star** provides a command line interface (CLI) for interacting with the virtual machine. The CLI allows users run, display and debug their assembly programs.

---

## Can you help me?
If you are in doubt about how to use the program, you can enter the command flag:
```bash
star --help
```
or
```bash
star -h
```

## Displaying the Version
To display the version of the **Star**, use the following command:
```bash
star --version
```
or
```bash
star -v
```
## Running a Program
To run a program file, use the following command:
```bash
star file.asm
```
or
```bash
star --file file.asm
```
or 
```bash
star -f file.asm
```

### Running a Program with debugging
In **Star**, you can run a program file with some debugging options.
#### Displaying the Registers
To display the final state of the registers after the end of the program, use the following command:
```bash
star file.asm --registers
```
or
```bash
star file.asm -r
```
#### Displaying the Symbol Table
To display the symbol table after the end of the program, use the following command:
```bash
star file.asm --symbol-table
```
or
```bash
star file.asm -s
```

### Saving the Binary of A Program
In **Star**, you can save a text file with a binary representation of a program.
It can be useful for learning purposes and for simulating the program in other environments, like a hardware simulator written in VHDL.
To save the binary of a program, use the following command:
```bash
star file.asm --binary output.txt
```
or
```bash
star file.asm -b output.txt
```