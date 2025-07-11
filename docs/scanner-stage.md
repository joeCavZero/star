# Scanner Stage

The scanner stage is responsible for reading the source code and converting it into tokens. This is the first step in the assembly process, where the raw text of the source code is transformed into a structured format that can be further processed by the parser.

## How the Scanner Works

The scanner reads the source code character by character, identifying and categorizing sequences of characters into tokens. It also records the position of each token within the source code, which is useful for error reporting and debugging.

To recognize different types of tokens, such as identifiers, literals, instructions, and processors, the scanner uses a set of rules defined by the language grammar. As it processes the input, the scanner distinguishes between these token types and generates a corresponding token stream.

When the scanner encounters a processor directive (for example, `@include` or `@define`), it resolves the processor type and handles it accordingly. This allows the assembler to support advanced features like file inclusion or macro definitions during the scanning phase.

You can read more about processors in the [processors documentation](/docs/processors.md).

The output of the scanner is a list of tokens, each with its type, value, and position information.

A well-designed scanner improves the reliability and maintainability of the assembler by ensuring that the source code is accurately and efficiently tokenized.

You can read more about how tokens are parsed in the [parser stage documentation](/docs/parser-stage.md).

## Example of Scanner Output
```python
.data
    string: .stringz "Hello World!"
.instr
    la $g, string
```
The scanner would produce tokens like:
```plaintext
[
    { type: 'directive', value: '.data', position: {file: 0, line: 1, column: 1} },
    { type: 'identifier', value: 'string', position: {file: 0, line: 2, column: 5} },
    { type: 'directive', value: '.stringz', position: {file: 0, line: 2, column: 13} },
    { type: 'string_literal', value: '"Hello World!"', position: {file: 0, line: 2, column: 22} },
    { type: 'directive', value: '.instr', position: {file: 0, line: 3, column: 1} },
    { type: 'instruction', value: 'la', position: {file: 0, line: 4, column: 1} },
    { type: 'general register', value: '$g', position: {file: 0, line: 4, column: 4} },
]
```