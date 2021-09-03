module AND(out, a, b);
    input a;
    input b;
    output out;

    wire c;

    nand(c, a, b);
    NOT not_instance(out, c);
endmodule
