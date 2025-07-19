# program to sum a vector of integers with a given size
.data
    vector: .word 10, 20, 30, 40, 50
    vector_size: .word 5

    string_result: .stringz "The sum of the vector is "
.instr
    la $a, vector
    la $b, vector_size
    lw $b, $b[0]
    li $g, 0
loop:
    # check if $b <= 0, if so, print and end
    beqa $b, $zero, print_and_end 
    blta $b, $zero, print_and_end

    # load the word from the vector into $c
    lab $c, $a  # load the byte in memory address $a into alt of $c
    inc $a      # increments $a by 1
    llb $c, $a  # load the byte in memory address $a into low of $c
    dec $a      # decrements $a by 1

    # sum the value in $c to $g
    add $g, $g, $c
    addi $a, $a, 2  # increment $a by 2 to point to the next word
    dec $b          # decrements the $b by 1 to count down the vector size

    ja loop     # jump address to loop
print_and_end:
    # prints the string result
    la $aux2, string_result
    li $aux1, 9
    mcall

    # prints the sum result
    li $aux1, 3
    move $aux2, $g
    mcall