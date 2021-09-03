module NOT(a, OUT);
    input a;
    output OUT;

    nand(OUT, a, a);
endmodule
