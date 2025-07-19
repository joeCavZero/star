# this program counts from a minimum to a maximum value
.data
    min: .word 1        # minimum value
    max: .word 10       # maximum value
    comma: .string ","  # comma string for output
.instr
    la $a, min          # load minimum into $a
    lw $a, $a[0]        # load minimun content into $a
    la $b, max          # load maximum into $b
    lw $b, $b[0]       # load maximum content into $b
loop:
    bgta $a, $b, end    # if $a > $b, jump to end

    li $aux1, 3         # mcall for print unsigned word
    move $aux2, $a      # move $a to $aux2
    mcall               # do the machine call
    
    beqa $a, $b, end    # if $a == $b, jump to end

    la $aux2, comma     # load comma into $aux2
    lb $aux2, $aux2[0]  # load comma content into $aux2
    li $aux1, 7         # mcall for print char  
    mcall               # do the machine call

    addi $a, $a, 1      # increment $a by 1
    ja loop             # jump address to loop
end: nope
    