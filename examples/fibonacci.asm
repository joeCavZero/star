# program to print Fibonacci numbers with a maximum value
.data
    max: .word 47000
    comma: .string ","
.instr
    li $a, 1
    li $b, 1
    la $d, max
    lw $d, $d[0]

    # prints the first number
    li $aux1, 3                 # mcall for print unsigned word
    move $aux2, $a              # move $a to $aux2
    mcall                       # do the machine call

    ja print_comma              # jump to print comma

    # prints the second number
    li $aux1, 3                 # mcall for print unsigned word
    move $aux2, $b              # move $b to $aux2
    mcall                       # do the machine call

loop:
    add $c, $a, $b              # c := a + b

    bneqa $carry, $zero, end    # if carry != 0 (not add overflow), jump to end
    bgtua $c, $d, end           # if c > max, jump to end

    ja print_comma              # jump to print comma

    # prints the fibonacci result number of the iteration
    li $aux1, 3                 # mcall for print unsigned word
    move $aux2, $c              # move $b to $aux2
    mcall                       # do the machine call

    move $a, $b                 # a := b
    move $b, $c                 # b := c

    ja loop                     # jump address to loop
print_comma:
    la $aux2, comma             # load comma into $aux2
    lb $aux2, $aux2[0]          # load comma content into $aux2
    li $aux1, 7                 # mcall for print char
    mcall                       # do the machine call
    ret                         # return from the function

end: nope