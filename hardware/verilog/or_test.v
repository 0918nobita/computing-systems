module ORTEST;
reg a, b;
wire out;

OR or_instance(a, b, out);

initial begin
    $dumpfile("or_test.vcd");
    $dumpvars(1, ORTEST);

        a = 0; b = 0;
    #10 a = 1;
    #10 a = 0; b = 1;
    #10 a = 1;
    #10 a = 0; b = 0;
    #10 $finish;
end

endmodule
