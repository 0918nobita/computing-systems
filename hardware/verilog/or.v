module OR(out, a, b);
    input a;
    input b;
    output out;

    wire a1;
    wire b1;

    NOT not_instance1(a1, a);
    NOT not_instance2(b1, b);
    nand(out, a1, b1);
endmodule
