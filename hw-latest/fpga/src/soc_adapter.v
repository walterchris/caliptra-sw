
module soc_adapter #(TAGW=16) (
input                   aclk,
input                   rstn,
input                   arvalid,
output wire             arready,
input [31:0]            araddr,
input [TAGW-1:0]        arid,
input [7:0]             arlen,
input [1:0]             arburst,
input [2:0]             arsize,

output reg              rvalid,
input                   rready,
output wire [31:0]      rdata,
output wire [1:0]       rresp,
output reg [TAGW-1:0]   rid,
output                  rlast,

input                   awvalid,
output                  awready,
input [31:0]            awaddr,
input [TAGW-1:0]        awid,
input [7:0]             awlen,
input [1:0]             awburst,
input [2:0]             awsize,

input [31:0]            wdata,
input [3:0]             wstrb,
input                   wvalid,
output                  wready,

output  reg             bvalid,
input                   bready,
output wire [1:0]       bresp,
output reg [TAGW-1:0]   bid,

// Caliptra SOC signals
input  wire [31:0] gpio_in,
output wire [31:0] gpio_out,
output wire [31:0] pauser,
output wire [255:0] cptra_obf_key
);

reg [7:0] mem [63:0];
reg [31:0] memdata;

wire [5:0] awaddr_masked;
assign awaddr_masked = awaddr[5:0];

always @ ( posedge aclk) begin
    if(!rstn) begin
        rvalid  <= 0;
        bvalid  <= 0;
    end
    else begin
        bid     <= awid;
        rid     <= arid;
        rvalid  <= arvalid;
        bvalid  <= awvalid;
    end

    if(arvalid) memdata <= {mem[araddr+3], mem[araddr+2], mem[araddr+1], mem[araddr]};
    if(awvalid) begin
        if (awaddr_masked != 8) begin
            if(wstrb[3]) mem[awaddr_masked+3] <= wdata[31:24];
            if(wstrb[2]) mem[awaddr_masked+2] <= wdata[23:16];
            if(wstrb[1]) mem[awaddr_masked+1] <= wdata[15:08];
            if(wstrb[0]) mem[awaddr_masked+0] <= wdata[07:00];
        end
    end

    mem[8]  <= gpio_in[7:0];
    mem[9]  <= gpio_in[15:8];
    mem[10] <= gpio_in[23:16];
    mem[11] <= gpio_in[31:24];
end

assign arready = 1'b1;
assign awready = 1'b1;
assign wready  = 1'b1;
assign rresp   = 2'b0;
assign bresp   = 2'b0;
assign rlast   = 1'b1;
assign rdata   = memdata;

assign gpio_out = {mem[3], mem[2], mem[1], mem[0]};
assign pauser = {mem[15], mem[14], mem[13], mem[12]};

genvar i;
generate
    for (i = 0; i < 32; i = i + 1) begin
        assign cptra_obf_key[(i*8)+7:(i*8)]   = mem[16 + i];
    end
endgenerate

endmodule
