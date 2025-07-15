@once // once says to the compiler that this macro should only be processed once

@define print_unsigned_byte(%arg) \
    li $aux1, 1 \
    move $aux2, %arg \
    mcall

@define print_byte(%arg) \
    li $aux1, 2 \
    move $aux2, %arg \
    mcall

@define print_unsigned_word(%arg) \
    li $aux1, 3 \
    move $aux2, %arg \
    mcall
    
@define print_word(%arg) \
    li $aux1, 4 \
    move $aux2, %arg \
    mcall
