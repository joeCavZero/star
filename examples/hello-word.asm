# This is a simple Hello World program in Star assembly language
.data
    string: .stringz "Hello, World!"    # string to print        
.instr
        li $aux1, 9         # mcall for print zero ended string
        la $aux2, string    # load address of string into $aux2
        mcall               # do the machine call