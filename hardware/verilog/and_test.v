module ANDTEST;
    reg a, b;
    wire out;

    AND and_instance(a, b, out);

    initial begin
        $dumpfile("and_test.vcd");
        $dumpvars(0, ANDTEST);

            a = 0; b = 0;
        #10 a = 1;
        #10 a = 0; b = 1;
        #10 a = 1;
        #10 a = 0; b = 0;
        #10 $finish;
    end
endmodule
