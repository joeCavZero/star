# This is a simple Hello World program in Star assembly language
# It is made using the brute character printing method
.data
    string: .stringz "Hello World!" # string to print (null-terminated)
.instr
start:
        la $g, string           # load address of label in $g
        li $a, 0                # load immediate 0 in $a
loop:   
        llb $a, $g              # load lower byte of address $g in $a
        beqa $a, $zero, end     # if $a == 0, jump to end
        
        # call for print char
        li $aux1, 7             # set mcall for print char
        move $aux2, $a          # move $a to $aux2
        mcall                   # do the machine call

        inc $g                  # increment address
        ja loop                 # jump address
end:    nope                    # ends the program