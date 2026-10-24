
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

    /* r24 and r25 are the first arg of the function that handle config address so move them into Y(r28 and r29)*/
    movw r28, r24

    /* Load config fields from memory at address Y and further */
    /* Use Pointer Z(r30 and r31) for storing data */
    ldd  r30, Y+0
    ldd  r31, Y+1
    /* Re-use r24 and r25 to store the number of LEDs */
    ldd  r24, Y+2
    ldd  r25, Y+3
    /* Use Pointer X(r26 and r27) to store port pointer */
    ldd  r26, Y+4
    ldd  r27, Y+5
    /* high and low port value calculated with the mask on port address*/
    ldd  r19, Y+6
    ldd  r20, Y+7

    /* No bytes? Return without transmitting. */
    mov r0, r24
    or   r24, r25
    breq .Ldone

    /* r21 = next port value; r23 = bit counter. */
    mov  r21, r20 /* Set port value to LOW */
    ldi  r23, 8 /* Init the data byte bit counter, will trigger Zero flag when counter to zero*/
    ld   r18, Z+

.Lhead:
    /* Begin bit sending, drive selected pin high. */
    st   X, r19 /* To drive pin high, store the value in r19 into the memory at address X */

    /* Keep high for a '1', or select low for a '0'. */
    sbrc r18, 7 
    mov  r21, r19

    dec  r23 /* decrement r23 (byte bit counter) */
    st   X, r21

    mov  r21, r20
    breq .Lnext_byte /* Run .Lnext_byte when Zero flag is triggered */

    rol  r18 /* Shift left so we go to next bit */

    /* Wait 3 cycles so at 16MHz, ~ 180ns*/
    rjmp .+0
    nop

    /* Ensure the output is low. */
    st   X, r20

    nop
    rjmp .+0
    rjmp .Lhead

.Lnext_byte:
    ldi  r23, 8 /* Reset counter */
    ld   r18, Z+
    st   X, r20 /* Set low on pin */
    nop

    sbiw r24, 1 /* Remove 1 to the number of led and trigger Zero flag if zero */
    brne .Lhead /* If no Zero flag, transfer next byte */

.Ldone:
    /* Leave the data pin low before returning. */
    pop  r29
    pop  r28
    ret

.size neopixel_tx, .-neopixel_tx
.section .note.GNU-stack,"",@progbits
