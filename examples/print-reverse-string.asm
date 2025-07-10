# program to read a string and print it in reverse order
.data
    string_enter: .stringz "Enter a string --> "            # prompt for input
    string_output: .stringz "The string in reverse is --> " # output message
    buffer: .space 256      # buffer for the string
.instr
    li $aux1, 9             # mcall for print zero ended string
    la $aux2, string_enter  # load address of prompt into $aux2
    mcall                   # do the machine call

    li $aux1, 15            # mcall for read string
    la $aux2, buffer        # load address of buffer into $aux2
    li $aux3, 256           # maximum size of the string
    mcall                   # do the machine call

    la $a, buffer
    li $b, 0
    li $c, 0
loop:
    llb $c, $a
    beqa $c, $zero, print
    inc $b
    inc $a
    ja loop
print:
    li $aux1, 9             # mcall for print zero ended string
    la $aux2, string_output # load address of output message into $aux2
    mcall                   # do the machine call
    print_loop:
        blta $b, $zero, end
        llb $c, $a

        li $aux1, 7          # mcall for print char
        move $aux2, $c       # move character to $aux2
        mcall                # do the machine call
        
        dec $b
        dec $a
        ja print_loop
end: nope