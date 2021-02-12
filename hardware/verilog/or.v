module ORTEST;
reg a, b;
wire out;

or(out, a, b);

initial begin
    $dumpfile("or.vcd");
    $dumpvars(0, ORTEST);

        a = 0; b = 0;
    #10 a = 1;
    #10 a = 0; b = 1;
    #10 a = 1;
    #10 a = 0; b = 0;
    #10 $finish;
end

endmodule
