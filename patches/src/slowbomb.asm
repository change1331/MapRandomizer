lorom

org $90C0AB  ; hook if bombs equipped
    JSR check
    nop #3

org $90C017  ; hook reg bomb timer
    JSR bombtimer
    
org $90FF00
checkpb:
    LDA $09D0           ;$[7E:09D0]     \
    BNE firebomb        ;               / if max powerbombs != 0: fire fake bomb
    sep #$02
    rts
check:
    LDA $09A2           ;$[7E:09A2]     \
    BIT #$1000          ;               } if bombs not equipped check pbs
    beq checkpb         ;               /
firebomb:
    rep #$02
    rts

bombtimer:
    pha
    lda $09A2
    bit #$1000
    beq suptimer
    pla
    STA $0CCC
    rts
suptimer:
    LDA #$0060
    STA $0CCC
    pla
    rts
