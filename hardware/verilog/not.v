module NOT(out, a);
    input a;
    output out;

    nand(out, a, a);
endmodule
