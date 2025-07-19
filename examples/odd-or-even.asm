# program to check if a number is odd or even
.data
    number: .word 23    # number to check
    string_odd: .stringz "The number is odd.\n"
    string_even: .stringz "The number is even.\n"
.instr
    la $a, number 
    lw $a, $a[0]
    andi $g, $a, 1
    beqa $g, $zero, even  # if $g == 0, jump to even
    ja odd
odd:
    la $aux2, string_odd  # load address of string_odd into $aux2
    ja print
even:
    la $aux2, string_even  # load address of string_even into $aux2
print: 
    li $aux1, 9           # mcall for print zero ended string
    mcall