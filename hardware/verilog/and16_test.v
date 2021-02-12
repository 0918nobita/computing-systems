module AND16TEST;
reg [15:0] a, b;
wire [15:0] out;

AND16 and16_instance(out, a, b);

integer i;

initial begin
    $dumpfile("and16_test.vcd");
    $dumpvars(1, AND16TEST);

    for (i = 0; i < 16; i = i + 1) begin
        a[i] = 0;
        b[i] = 0;
    end

    #10 a[0] = 1;
    #10 a[0] = 0; b[0] = 1;
    #10 a[0] = 1;
    #10 a[0] = 0; b[0] = 0;
    #10 $finish;
end

endmodule
