module AND16(out, a, b);
    input [15:0] a, b;
    output [15:0] out;

    genvar i;
    for (i = 0; i < 16; i = i + 1) begin
        AND and_instance(out[i], a[i], b[i]);
    end
endmodule
