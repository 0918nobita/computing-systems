module NOTTEST;
    reg a;
    wire out;

    NOT not_instance(out, a);

    initial begin
        $dumpfile("not_test.vcd");
        $dumpvars(0, NOTTEST);

            a = 0;
        #10 a = 1;
        #10 a = 0;
        #10 $finish;
    end
endmodule
