// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.
@8192
D = A
@2
M = D
(BEGIN)
    @0
    M = 0
    @KBD
    D = M
    @BLACK
    D; JNE
    @WHITE
    0; JMP
(BLACK)
    @SCREEN
    D = A
    @0
    A = M + D
    M = -1
    @0
    D = M + 1
    M = M + 1
    @2
    D = D - M
    @BLACK
    D; JLT
    @BEGIN
    0; JMP
(WHITE)
    @SCREEN
    D = A
    @0
    A = M + D
    M = 0 
    @0
    D = M + 1
    M = M + 1
    @2
    D = D - M
    @WHITE
    D; JLT
    @BEGIN
    0; JMP