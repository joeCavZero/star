# program to print numbers from 1 to N with commas in between
.data
    str_hello: .stringz "Hello! How many numbers do you want to print? "
    comma: .string ","
.instr
    # print hello message
    li $aux1, 9             
    la $aux2, str_hello
    mcall

    # read unsigned word
    li $aux1, 11
    mcall

    move $a, $aux2      # save the input number in $a
    li $b, 1            # initialize $b to M

    bgtua $b, $a, end   # if $b > $a then end

    # print unsigned word
    li $aux1, 3
    move $aux2, $b
    mcall

    inc $b              # increment $b by 1

loop:

    bgtua $b, $a, end   # if $b > $a then end

    # print comma
    la $g, comma
    lb $g, $g[0]
    li $aux1, 7
    move $aux2, $g
    mcall

    # print unsigned word
    li $aux1, 3
    move $aux2, $b
    mcall

    inc $b          # increment $b by 1
    ja loop         # jump to loop

end: nope