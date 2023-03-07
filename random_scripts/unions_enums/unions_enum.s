	.text
	.file	"unions_enum.fc6d9eca-cgu.0"
	.section	".text._ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h5b987328ac7be775E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h5b987328ac7be775E,@function
_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h5b987328ac7be775E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdi, (%rsp)
	movq	(%rsp), %rax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h5b987328ac7be775E, .Lfunc_end0-_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h5b987328ac7be775E
	.cfi_endproc

	.section	.text._ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hda11a8643ef68a7bE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hda11a8643ef68a7bE,@function
_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hda11a8643ef68a7bE:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	callq	_ZN4core3ops8function6FnOnce9call_once17hbc01ae1845877d15E
	#APP
	#NO_APP
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hda11a8643ef68a7bE, .Lfunc_end1-_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hda11a8643ef68a7bE
	.cfi_endproc

	.section	.text._ZN3std2rt10lang_start17h03fb6faee1baade7E,"ax",@progbits
	.hidden	_ZN3std2rt10lang_start17h03fb6faee1baade7E
	.globl	_ZN3std2rt10lang_start17h03fb6faee1baade7E
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start17h03fb6faee1baade7E,@function
_ZN3std2rt10lang_start17h03fb6faee1baade7E:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movl	%ecx, %eax
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, 16(%rsp)
	leaq	16(%rsp), %rdi
	leaq	.L__unnamed_1(%rip), %rsi
	movzbl	%al, %r8d
	callq	*_ZN3std2rt19lang_start_internal17h79190e3a877a769dE@GOTPCREL(%rip)
	movq	%rax, 8(%rsp)
	movq	8(%rsp), %rax
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end2:
	.size	_ZN3std2rt10lang_start17h03fb6faee1baade7E, .Lfunc_end2-_ZN3std2rt10lang_start17h03fb6faee1baade7E
	.cfi_endproc

	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h07ceb2f806a9e1e2E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h07ceb2f806a9e1e2E,@function
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h07ceb2f806a9e1e2E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hda11a8643ef68a7bE
	callq	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h565057b288936890E
	movb	%al, 7(%rsp)
	movzbl	7(%rsp), %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end3:
	.size	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h07ceb2f806a9e1e2E, .Lfunc_end3-_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h07ceb2f806a9e1e2E
	.cfi_endproc

	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h4f08c2635d4db17aE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h4f08c2635d4db17aE,@function
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h4f08c2635d4db17aE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN4core3ops8function6FnOnce9call_once17hd61c61989472cc63E
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end4:
	.size	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h4f08c2635d4db17aE, .Lfunc_end4-_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h4f08c2635d4db17aE
	.cfi_endproc

	.section	.text._ZN4core3ops8function6FnOnce9call_once17hbc01ae1845877d15E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce9call_once17hbc01ae1845877d15E,@function
_ZN4core3ops8function6FnOnce9call_once17hbc01ae1845877d15E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	*%rdi
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end5:
	.size	_ZN4core3ops8function6FnOnce9call_once17hbc01ae1845877d15E, .Lfunc_end5-_ZN4core3ops8function6FnOnce9call_once17hbc01ae1845877d15E
	.cfi_endproc

	.section	.text._ZN4core3ops8function6FnOnce9call_once17hd61c61989472cc63E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce9call_once17hd61c61989472cc63E,@function
_ZN4core3ops8function6FnOnce9call_once17hd61c61989472cc63E:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, 8(%rsp)
.Ltmp0:
	leaq	8(%rsp), %rdi
	callq	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h07ceb2f806a9e1e2E
.Ltmp1:
	movl	%eax, 4(%rsp)
	jmp	.LBB6_3
.LBB6_1:
	movq	24(%rsp), %rdi
	callq	_Unwind_Resume@PLT
	ud2
.LBB6_2:
.Ltmp2:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 24(%rsp)
	movl	%eax, 32(%rsp)
	jmp	.LBB6_1
.LBB6_3:
	movl	4(%rsp), %eax
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end6:
	.size	_ZN4core3ops8function6FnOnce9call_once17hd61c61989472cc63E, .Lfunc_end6-_ZN4core3ops8function6FnOnce9call_once17hd61c61989472cc63E
	.cfi_endproc
	.section	.gcc_except_table._ZN4core3ops8function6FnOnce9call_once17hd61c61989472cc63E,"a",@progbits
	.p2align	2
GCC_except_table6:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Ltmp0
	.uleb128 .Ltmp2-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp1-.Lfunc_begin0
	.uleb128 .Lfunc_end6-.Ltmp1
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2

	.section	".text._ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h9ac8dab5e9f008b9E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h9ac8dab5e9f008b9E,@function
_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h9ac8dab5e9f008b9E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h0c958011e61473e0E
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end7:
	.size	_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h9ac8dab5e9f008b9E, .Lfunc_end7-_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h9ac8dab5e9f008b9E
	.cfi_endproc

	.section	".text._ZN4core3ptr42drop_in_place$LT$unions_enum..NumOrStr$GT$17h4933b37fc3e06e3cE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr42drop_in_place$LT$unions_enum..NumOrStr$GT$17h4933b37fc3e06e3cE,@function
_ZN4core3ptr42drop_in_place$LT$unions_enum..NumOrStr$GT$17h4933b37fc3e06e3cE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdi, (%rsp)
	movq	8(%rdi), %rdx
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, %rdx
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB8_2
.LBB8_1:
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.LBB8_2:
	.cfi_def_cfa_offset 16
	movq	(%rsp), %rdi
	callq	_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h9ac8dab5e9f008b9E
	jmp	.LBB8_1
.Lfunc_end8:
	.size	_ZN4core3ptr42drop_in_place$LT$unions_enum..NumOrStr$GT$17h4933b37fc3e06e3cE, .Lfunc_end8-_ZN4core3ptr42drop_in_place$LT$unions_enum..NumOrStr$GT$17h4933b37fc3e06e3cE
	.cfi_endproc

	.section	".text._ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h0c958011e61473e0E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h0c958011e61473e0E,@function
_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h0c958011e61473e0E:
.Lfunc_begin1:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception1
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, (%rsp)
.Ltmp3:
	callq	_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hc17913073f2490a2E
.Ltmp4:
	jmp	.LBB9_3
.LBB9_1:
.Ltmp6:
	movq	(%rsp), %rdi
	callq	_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h2eb650e467a3ba59E
.Ltmp7:
	jmp	.LBB9_5
.LBB9_2:
.Ltmp5:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 8(%rsp)
	movl	%eax, 16(%rsp)
	jmp	.LBB9_1
.LBB9_3:
	movq	(%rsp), %rdi
	callq	_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h2eb650e467a3ba59E
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB9_4:
	.cfi_def_cfa_offset 32
.Ltmp8:
	movq	_ZN4core9panicking15panic_no_unwind17hb07180a78460155fE@GOTPCREL(%rip), %rax
	callq	*%rax
	ud2
.LBB9_5:
	movq	8(%rsp), %rdi
	callq	_Unwind_Resume@PLT
	ud2
.Lfunc_end9:
	.size	_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h0c958011e61473e0E, .Lfunc_end9-_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h0c958011e61473e0E
	.cfi_endproc
	.section	".gcc_except_table._ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h0c958011e61473e0E","a",@progbits
	.p2align	2
GCC_except_table9:
.Lexception1:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end1-.Lcst_begin1
.Lcst_begin1:
	.uleb128 .Ltmp3-.Lfunc_begin1
	.uleb128 .Ltmp4-.Ltmp3
	.uleb128 .Ltmp5-.Lfunc_begin1
	.byte	0
	.uleb128 .Ltmp6-.Lfunc_begin1
	.uleb128 .Ltmp7-.Ltmp6
	.uleb128 .Ltmp8-.Lfunc_begin1
	.byte	0
	.uleb128 .Ltmp7-.Lfunc_begin1
	.uleb128 .Lfunc_end9-.Ltmp7
	.byte	0
	.byte	0
.Lcst_end1:
	.p2align	2

	.section	".text._ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h2eb650e467a3ba59E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h2eb650e467a3ba59E,@function
_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h2eb650e467a3ba59E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h4adaa001728e7157E
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end10:
	.size	_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h2eb650e467a3ba59E, .Lfunc_end10-_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h2eb650e467a3ba59E
	.cfi_endproc

	.section	".text._ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17h68301913262da1f2E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17h68301913262da1f2E,@function
_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17h68301913262da1f2E:
	.cfi_startproc
	subq	$3, %rsp
	.cfi_def_cfa_offset 11
	cmpq	%rsi, %rdi
	sete	%al
	andb	$1, %al
	movb	%al, 2(%rsp)
	movb	2(%rsp), %al
	movb	%al, (%rsp)
	cmpb	$2, %al
	jne	.LBB11_2
	movb	$2, 1(%rsp)
	jmp	.LBB11_3
.LBB11_2:
	movb	(%rsp), %al
	cmpb	$1, %al
	sete	%al
	andb	$1, %al
	movb	%al, 1(%rsp)
.LBB11_3:
	movb	1(%rsp), %al
	addq	$3, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end11:
	.size	_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17h68301913262da1f2E, .Lfunc_end11-_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17h68301913262da1f2E
	.cfi_endproc

	.section	".text._ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h3ce84c8b5a93eaa4E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h3ce84c8b5a93eaa4E,@function
_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h3ce84c8b5a93eaa4E:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	$0, 32(%rsp)
	movq	32(%rsp), %rax
	movq	%rax, 24(%rsp)
	movq	24(%rsp), %rax
	movq	%rax, 16(%rsp)
	movq	16(%rsp), %rsi
	callq	_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17h68301913262da1f2E
	movb	%al, 15(%rsp)
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpb	$2, 15(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB12_2
	movb	$0, 14(%rsp)
	jmp	.LBB12_3
.LBB12_2:
	movb	15(%rsp), %al
	andb	$1, %al
	movb	%al, 14(%rsp)
.LBB12_3:
	movb	14(%rsp), %al
	andb	$1, %al
	movzbl	%al, %eax
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end12:
	.size	_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h3ce84c8b5a93eaa4E, .Lfunc_end12-_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h3ce84c8b5a93eaa4E
	.cfi_endproc

	.section	".text._ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h77b11c61d7a426f7E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h77b11c61d7a426f7E,@function
_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h77b11c61d7a426f7E:
	.cfi_startproc
	retq
.Lfunc_end13:
	.size	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h77b11c61d7a426f7E, .Lfunc_end13-_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h77b11c61d7a426f7E
	.cfi_endproc

	.section	.text._ZN4core4hint21unreachable_unchecked17h6abd4be84ae765e5E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core4hint21unreachable_unchecked17h6abd4be84ae765e5E,@function
_ZN4core4hint21unreachable_unchecked17h6abd4be84ae765e5E:
	.cfi_startproc
	ud2
.Lfunc_end14:
	.size	_ZN4core4hint21unreachable_unchecked17h6abd4be84ae765e5E, .Lfunc_end14-_ZN4core4hint21unreachable_unchecked17h6abd4be84ae765e5E
	.cfi_endproc

	.section	.text._ZN4core5alloc6layout6Layout5array5inner17h5ae6b7e56a1d8d7bE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core5alloc6layout6Layout5array5inner17h5ae6b7e56a1d8d7bE,@function
_ZN4core5alloc6layout6Layout5array5inner17h5ae6b7e56a1d8d7bE:
	.cfi_startproc
	subq	$104, %rsp
	.cfi_def_cfa_offset 112
	movq	%rdi, 16(%rsp)
	movq	%rsi, 24(%rsp)
	movq	%rdx, 32(%rsp)
	cmpq	$0, %rdi
	jne	.LBB15_2
	movb	$0, 63(%rsp)
	jmp	.LBB15_5
.LBB15_2:
	movq	16(%rsp), %rax
	movq	24(%rsp), %rcx
	movq	%rcx, 80(%rsp)
	movq	80(%rsp), %rdx
	subq	$1, %rdx
	movabsq	$9223372036854775807, %rcx
	subq	%rdx, %rcx
	movq	%rcx, 8(%rsp)
	cmpq	$0, %rax
	sete	%al
	testb	$1, %al
	jne	.LBB15_4
	movq	16(%rsp), %rcx
	movq	8(%rsp), %rax
	xorl	%edx, %edx
	divq	%rcx
	movq	%rax, %rcx
	movq	32(%rsp), %rax
	cmpq	%rcx, %rax
	seta	%al
	andb	$1, %al
	movb	%al, 63(%rsp)
	jmp	.LBB15_5
.LBB15_4:
	leaq	str.0(%rip), %rdi
	leaq	.L__unnamed_2(%rip), %rdx
	movq	_ZN4core9panicking5panic17h341545107301821dE@GOTPCREL(%rip), %rax
	movl	$25, %esi
	callq	*%rax
	ud2
.LBB15_5:
	testb	$1, 63(%rsp)
	jne	.LBB15_7
	movq	24(%rsp), %rax
	movq	32(%rsp), %rdx
	movq	16(%rsp), %rcx
	imulq	%rdx, %rcx
	movq	%rax, 88(%rsp)
	movq	88(%rsp), %rax
	movq	%rax, 96(%rsp)
	movq	96(%rsp), %rax
	movq	%rcx, 64(%rsp)
	movq	%rax, 72(%rsp)
	movq	64(%rsp), %rcx
	movq	72(%rsp), %rax
	movq	%rcx, 40(%rsp)
	movq	%rax, 48(%rsp)
	jmp	.LBB15_8
.LBB15_7:
	movq	$0, 48(%rsp)
.LBB15_8:
	movq	40(%rsp), %rax
	movq	48(%rsp), %rdx
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end15:
	.size	_ZN4core5alloc6layout6Layout5array5inner17h5ae6b7e56a1d8d7bE, .Lfunc_end15-_ZN4core5alloc6layout6Layout5array5inner17h5ae6b7e56a1d8d7bE
	.cfi_endproc

	.section	".text._ZN4core6result19Result$LT$T$C$E$GT$16unwrap_unchecked17h0faad4aefaf09a15E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core6result19Result$LT$T$C$E$GT$16unwrap_unchecked17h0faad4aefaf09a15E,@function
_ZN4core6result19Result$LT$T$C$E$GT$16unwrap_unchecked17h0faad4aefaf09a15E:
.Lfunc_begin2:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception2
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, 24(%rsp)
	movq	%rsi, 32(%rsp)
	xorl	%eax, %eax
	movl	$1, %ecx
	cmpq	$0, 32(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	jne	.LBB16_2
	movq	24(%rsp), %rax
	movq	%rax, 8(%rsp)
	movq	32(%rsp), %rax
	movq	%rax, 16(%rsp)
	xorl	%eax, %eax
	movl	$1, %ecx
	cmpq	$0, 32(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	je	.LBB16_10
	jmp	.LBB16_11
.LBB16_2:
.Ltmp9:
	callq	_ZN4core4hint21unreachable_unchecked17h6abd4be84ae765e5E
.Ltmp10:
	jmp	.LBB16_5
.LBB16_3:
	xorl	%eax, %eax
	movl	$1, %ecx
	cmpq	$0, 32(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$0, %rax
	je	.LBB16_6
	jmp	.LBB16_7
.LBB16_4:
.Ltmp11:
	movq	%rax, %rcx
	movl	%edx, %eax
	movq	%rcx, 40(%rsp)
	movl	%eax, 48(%rsp)
	jmp	.LBB16_3
.LBB16_5:
	ud2
.LBB16_6:
	movb	$1, %al
	testb	$1, %al
	jne	.LBB16_9
	jmp	.LBB16_8
.LBB16_7:
	jmp	.LBB16_8
.LBB16_8:
	movq	40(%rsp), %rdi
	callq	_Unwind_Resume@PLT
	ud2
.LBB16_9:
	jmp	.LBB16_8
.LBB16_10:
	movq	16(%rsp), %rdx
	movq	8(%rsp), %rax
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB16_11:
	.cfi_def_cfa_offset 64
	jmp	.LBB16_10
.Lfunc_end16:
	.size	_ZN4core6result19Result$LT$T$C$E$GT$16unwrap_unchecked17h0faad4aefaf09a15E, .Lfunc_end16-_ZN4core6result19Result$LT$T$C$E$GT$16unwrap_unchecked17h0faad4aefaf09a15E
	.cfi_endproc
	.section	".gcc_except_table._ZN4core6result19Result$LT$T$C$E$GT$16unwrap_unchecked17h0faad4aefaf09a15E","a",@progbits
	.p2align	2
GCC_except_table16:
.Lexception2:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end2-.Lcst_begin2
.Lcst_begin2:
	.uleb128 .Ltmp9-.Lfunc_begin2
	.uleb128 .Ltmp10-.Ltmp9
	.uleb128 .Ltmp11-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp10-.Lfunc_begin2
	.uleb128 .Lfunc_end16-.Ltmp10
	.byte	0
	.byte	0
.Lcst_end2:
	.p2align	2

	.section	".text._ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h7d48192b327036a6E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h7d48192b327036a6E,@function
_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h7d48192b327036a6E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17h5b987328ac7be775E
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end17:
	.size	_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h7d48192b327036a6E, .Lfunc_end17-_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h7d48192b327036a6E
	.cfi_endproc

	.section	".text._ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h565057b288936890E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h565057b288936890E,@function
_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h565057b288936890E:
	.cfi_startproc
	xorl	%eax, %eax
	retq
.Lfunc_end18:
	.size	_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h565057b288936890E, .Lfunc_end18-_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h565057b288936890E
	.cfi_endproc

	.section	".text._ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h64250800749e6547E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h64250800749e6547E,@function
_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h64250800749e6547E:
	.cfi_startproc
	subq	$104, %rsp
	.cfi_def_cfa_offset 112
	movq	%rsi, 24(%rsp)
	movq	%rdi, 32(%rsp)
	movq	%rdi, 40(%rsp)
	xorl	%eax, %eax
	testb	$1, %al
	jne	.LBB19_2
	movq	24(%rsp), %rax
	cmpq	$0, (%rax)
	sete	%al
	andb	$1, %al
	movb	%al, 55(%rsp)
	jmp	.LBB19_3
.LBB19_2:
	movb	$1, 55(%rsp)
.LBB19_3:
	testb	$1, 55(%rsp)
	jne	.LBB19_5
	movq	24(%rsp), %rax
	movq	(%rax), %rdx
	movq	$1, 96(%rsp)
	movq	96(%rsp), %rsi
	movl	$1, %edi
	callq	_ZN4core5alloc6layout6Layout5array5inner17h5ae6b7e56a1d8d7bE
	movq	%rax, %rdi
	movq	%rdx, %rsi
	leaq	.L__unnamed_3(%rip), %rdx
	callq	_ZN4core6result19Result$LT$T$C$E$GT$16unwrap_unchecked17h0faad4aefaf09a15E
	movq	%rax, %rcx
	movq	24(%rsp), %rax
	movq	%rcx, 8(%rsp)
	movq	%rdx, 16(%rsp)
	movq	8(%rax), %rax
	movq	%rax, 88(%rsp)
	movq	88(%rsp), %rax
	movq	%rax, 80(%rsp)
	movq	80(%rsp), %rdi
	callq	_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h7d48192b327036a6E
	movq	8(%rsp), %rcx
	movq	16(%rsp), %rdx
	movq	%rax, %rsi
	movq	32(%rsp), %rax
	movq	%rsi, 56(%rsp)
	movq	%rcx, 64(%rsp)
	movq	%rdx, 72(%rsp)
	movq	56(%rsp), %rcx
	movq	%rcx, (%rax)
	movq	64(%rsp), %rcx
	movq	%rcx, 8(%rax)
	movq	72(%rsp), %rcx
	movq	%rcx, 16(%rax)
	jmp	.LBB19_6
.LBB19_5:
	movq	32(%rsp), %rax
	movq	$0, 16(%rax)
.LBB19_6:
	movq	40(%rsp), %rax
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end19:
	.size	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h64250800749e6547E, .Lfunc_end19-_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h64250800749e6547E
	.cfi_endproc

	.section	".text._ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hf8d44dc8f20c74feE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hf8d44dc8f20c74feE,@function
_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hf8d44dc8f20c74feE:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rsi, 8(%rsp)
	movq	%rdx, 16(%rsp)
	movq	%rcx, 24(%rsp)
	cmpq	$0, 16(%rsp)
	jne	.LBB20_2
	jmp	.LBB20_3
.LBB20_2:
	movq	8(%rsp), %rdi
	movq	16(%rsp), %rcx
	movq	24(%rsp), %rax
	movq	%rcx, 32(%rsp)
	movq	%rax, 40(%rsp)
	movq	32(%rsp), %rsi
	movq	40(%rsp), %rax
	movq	%rax, 48(%rsp)
	movq	48(%rsp), %rdx
	callq	*__rust_dealloc@GOTPCREL(%rip)
.LBB20_3:
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end20:
	.size	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hf8d44dc8f20c74feE, .Lfunc_end20-_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hf8d44dc8f20c74feE
	.cfi_endproc

	.section	".text._ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hc17913073f2490a2E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hc17913073f2490a2E,@function
_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hc17913073f2490a2E:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, 8(%rsp)
	movq	8(%rdi), %rdi
	movq	%rdi, 16(%rsp)
	callq	_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h3ce84c8b5a93eaa4E
	movq	8(%rsp), %rdi
	movq	16(%rsp), %rcx
	movq	16(%rdi), %rax
	movq	%rcx, 40(%rsp)
	movq	%rax, 48(%rsp)
	movq	40(%rsp), %rcx
	movq	48(%rsp), %rax
	movq	%rcx, 24(%rsp)
	movq	%rax, 32(%rsp)
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end21:
	.size	_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hc17913073f2490a2E, .Lfunc_end21-_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hc17913073f2490a2E
	.cfi_endproc

	.section	".text._ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h4adaa001728e7157E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h4adaa001728e7157E,@function
_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h4adaa001728e7157E:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, %rsi
	movq	%rsi, 8(%rsp)
	leaq	16(%rsp), %rdi
	callq	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h64250800749e6547E
	movl	$1, %eax
	xorl	%ecx, %ecx
	cmpq	$0, 32(%rsp)
	cmoveq	%rcx, %rax
	cmpq	$1, %rax
	jne	.LBB22_2
	movq	8(%rsp), %rdi
	movq	16(%rsp), %rsi
	movq	24(%rsp), %rdx
	movq	32(%rsp), %rcx
	callq	_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hf8d44dc8f20c74feE
.LBB22_2:
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end22:
	.size	_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h4adaa001728e7157E, .Lfunc_end22-_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h4adaa001728e7157E
	.cfi_endproc

	.section	.text._ZN11unions_enum4main17h119f56b00d31b0cbE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN11unions_enum4main17h119f56b00d31b0cbE,@function
_ZN11unions_enum4main17h119f56b00d31b0cbE:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movw	$1234, (%rsp)
	movq	$0, 8(%rsp)
	movq	%rsp, %rdi
	callq	_ZN4core3ptr42drop_in_place$LT$unions_enum..NumOrStr$GT$17h4933b37fc3e06e3cE
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end23:
	.size	_ZN11unions_enum4main17h119f56b00d31b0cbE, .Lfunc_end23-_ZN11unions_enum4main17h119f56b00d31b0cbE
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rsi, %rdx
	movslq	%edi, %rsi
	leaq	_ZN11unions_enum4main17h119f56b00d31b0cbE(%rip), %rdi
	xorl	%ecx, %ecx
	callq	_ZN3std2rt10lang_start17h03fb6faee1baade7E
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end24:
	.size	main, .Lfunc_end24-main
	.cfi_endproc

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	3
.L__unnamed_1:
	.quad	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h77b11c61d7a426f7E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h4f08c2635d4db17aE
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h07ceb2f806a9e1e2E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h07ceb2f806a9e1e2E
	.size	.L__unnamed_1, 48

	.type	.L__unnamed_4,@object
	.section	.rodata..L__unnamed_4,"a",@progbits
.L__unnamed_4:
	.ascii	"/rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/alloc/layout.rs"
	.size	.L__unnamed_4, 80

	.type	.L__unnamed_2,@object
	.section	.data.rel.ro..L__unnamed_2,"aw",@progbits
	.p2align	3
.L__unnamed_2:
	.quad	.L__unnamed_4
	.asciz	"P\000\000\000\000\000\000\000\304\001\000\000)\000\000"
	.size	.L__unnamed_2, 24

	.type	str.0,@object
	.section	.rodata.str.0,"a",@progbits
	.p2align	4
str.0:
	.ascii	"attempt to divide by zero"
	.size	str.0, 25

	.type	.L__unnamed_5,@object
	.section	.rodata..L__unnamed_5,"a",@progbits
.L__unnamed_5:
	.ascii	"/rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/raw_vec.rs"
	.size	.L__unnamed_5, 76

	.type	.L__unnamed_3,@object
	.section	.data.rel.ro..L__unnamed_3,"aw",@progbits
	.p2align	3
.L__unnamed_3:
	.quad	.L__unnamed_5
	.asciz	"L\000\000\000\000\000\000\000\367\000\000\000;\000\000"
	.size	.L__unnamed_3, 24

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"aGw",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.section	".note.GNU-stack","",@progbits
