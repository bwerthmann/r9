.section .text.entry
.globl start
start:
	bnez	a0, 3f
	la	sp, stack	// set the stack pointer
	li	t0, 4096 * 4
	add	sp, sp, t0	// add stack length

	// clear bss section
	la 		a0, sbss // bss start
	la		a1, end  // bss end
	bgeu	a0, a1, 2f
1:
	sd		zero, (a0)
	addi	a0, a0, 8
	bltu	a0, a1, 1b
2:

	call	main9
3:
	wfi
	j	1b

.bss
.balign 4096
stack:	.space 4096 * 4
