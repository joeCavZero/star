# this program counts from a minimum to a maximum value
.data
    min: .word 1        # minimum value
    max: .word 10       # maximum value
    comma: .string ","  # comma string for output
.instr
    lw $a, min[0]       # loads a minimum value into $a
    lw $b, max[0]       # loads a maximum value into $b
loop:
    bgta $a, $b, end    # if $a > $b, jump to end

    li $aux1, 3         # mcall for print unsigned word
    move $aux2, $a      # move $a to $aux2
    mcall               # do the machine call
    
    beqa $a, $b, end    # if $a == $b, jump to end

    lb $aux2, comma[0]  # load comma into $aux2
    li $aux1, 7         # mcall for print char  
    mcall               # do the machine call

    addi $a, $a, 1      # increment $a by 1
    ja loop             # jump address to loop
end: nope
    