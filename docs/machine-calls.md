# Machine Calls

Machine calls are special instructions that allow a program to interact with the system and perform I/O operations. Each machine call is identified by a unique call number and can use up to three auxiliary registers as parameters:

- **$aux1**: The first auxiliary register, used to specify the operation or provide additional parameters.
- **$aux2**: The second auxiliary register, often used to hold data or addresses related to the operation.
- **$aux3**: The third auxiliary register, used for additional data or addresses when necessary.
    
| call name               | $aux1 | $aux2           | $aux3          | description                                                                                                 |
|:----------------------- |:-----:|:---------------:|:--------------:|:------------------------------------------------------------------------------------------------------------|
| print unsigned byte     | 1     | number          | ~              | Prints the lower 8 bits of a 16-bit number as an unsigned byte.                                             |
| print signed byte       | 2     | number          | ~              | Prints the lower 8 bits of a 16-bit number as a signed byte.                                                |
| print unsigned word     | 3     | number          | ~              | Prints a 16-bit unsigned integer.                                                                           |
| print signed word       | 4     | number          | ~              | Prints a 16-bit signed integer.                                                                             |
| print unsigned double   | 5     | low             | high           | Prints a 32-bit unsigned integer, where `low` and `high` are the lower and upper 16 bits, respectively.     |
| print signed double     | 6     | low             | high           | Prints a 32-bit signed integer, where `low` and `high` are the lower and upper 16 bits, respectively.       |
| print char              | 7     | char            | ~              | Prints the lower 8 bits of $aux2 as a character.                                                            |
| print string            | 8     | address         | length         | Prints a string from memory starting at `address` with the given `length` (does not stop at zero byte).     |
| print string zero       | 9     | address         | ~              | Prints a string from memory starting at `address` until the first zero byte (`\0`).                         |
| read byte               | 10    | ~               | ~              | Reads a byte (u8) from user input and stores it in $aux2 (lower 8 bits).                                    |
| read word               | 11    | ~               | ~              | Reads a 16-bit unsigned integer from user input and stores it in $aux2.                                     |
| read double             | 12    | ~               | ~              | Reads a 32-bit unsigned integer from user input and stores lower 16 bits in $aux2, upper 16 bits in $aux3.  |
| read char               | 13    | ~               | ~              | Reads a single character from user input and stores its code in $aux2.                                      |
| read string             | 14    | address         | max length     | Reads a string from user input, stores it at `address` (up to `max length`), and stores the length in $aux2.|
| read string zero        | 15    | address         | max length     | Reads a string from user input, stores it at `address` (up to `max length`), always adds a zero byte at end.|
| exit                    | 16    | ~               | ~              | Exits the program.                                                                                          |
| print instruction       | 17    | pc              | ~              | Prints the 16-bit instruction at the given program counter (`pc`). Useful for debugging.                    |
| sleep                   | 18    | millis          | ~              | Pauses execution for the given number of milliseconds.                                                      |
| random                  | 19    | ~               | ~              | Generates a random number and stores it in $aux2.                                                           |

> For more information about the execution of machine calls, see the [execution stage documentation](/docs/execution-stage.md).