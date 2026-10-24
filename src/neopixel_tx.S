
/*
 * ATmega328P, 16 MHz, WS2812 800 kHz.
 *
 * void neopixel_tx(const TxConfig *cfg);
 *
 * TxConfig layout on this 8-bit AVR:
 *   offset 0: data pointer (2 bytes)
 *   offset 2: byte count   (2 bytes)
 *   offset 4: PORT pointer (2 bytes)
 *   offset 6: hi           (1 byte)
 *   offset 7: lo           (1 byte)
 *
 * The caller disables interrupts before calling this function.
 */

.section .text
.global neopixel_tx
.type neopixel_tx, @function

neopixel_tx:
    /* Preserve Y: r28:r29 are callee-saved in the AVR-GCC ABI. */
    push r28
    push r29

    /* Y points to the config structure. */
    movw r28, r24

    /* Load config fields. */
    ldd  r30, Y+0              /* ZL = data pointer low */
    ldd  r31, Y+1              /* ZH = data pointer high */
    ldd  r24, Y+2              /* count low */
    ldd  r25, Y+3              /* count high */
    ldd  r26, Y+4              /* XL = PORT pointer low */
    ldd  r27, Y+5              /* XH = PORT pointer high */
    ldd  r19, Y+6              /* hi port value */
    ldd  r20, Y+7              /* lo port value */

    /* No bytes? Return without transmitting. */
    mov r0, r24
    or   r24, r25
    breq .Ldone

    /* r21 = next port value; r23 = bit counter. */
    mov  r21, r20
    ldi  r23, 8
    ld   r18, Z+

.Lhead:
    /* Cycle 0: begin bit, drive selected pin high. */
    st   X, r19

    /* Keep high for a '1', or select low for a '0'. */
    sbrc r18, 7
    mov  r21, r19

    dec  r23
    /* Cycle 5: output the selected intermediate level. */
    st   X, r21

    mov  r21, r20
    breq .Lnext_byte

    rol  r18
    rjmp .+0
    nop

    /* Cycle 13: ensure the output is low. */
    st   X, r20

    nop
    rjmp .+0
    rjmp .Lhead

.Lnext_byte:
    ldi  r23, 8
    ld   r18, Z+
    st   X, r20
    nop

    sbiw r24, 1
    brne .Lhead

.Ldone:
    /* Leave the data pin low before returning. */
    pop  r29
    pop  r28
    ret

.size neopixel_tx, .-neopixel_tx
.section .note.GNU-stack,"",@progbits
