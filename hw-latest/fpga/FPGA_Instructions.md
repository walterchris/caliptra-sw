_*SPDX-License-Identifier: Apache-2.0<BR>
<BR>
<BR>
Licensed under the Apache License, Version 2.0 (the "License");<BR>
you may not use this file except in compliance with the License.<BR>
You may obtain a copy of the License at<BR>
<BR>
http://www.apache.org/licenses/LICENSE-2.0 <BR>
<BR>
Unless required by applicable law or agreed to in writing, software<BR>
distributed under the License is distributed on an "AS IS" BASIS,<BR>
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.<BR>
See the License for the specific language governing permissions and<BR>
limitations under the License.*_<BR>

# **Caliptra FPGA Guide** #

### Requirements: ###
 - Vivado
   - `Version v2022.2`
 - FPGA
   - `ZCU104 Development Board`
   - https://www.xilinx.com/products/boards-and-kits/zcu104.html
 - [Not fully enabled] JTAG debugger
   - `Olimex ARM-USB-TINY-H`

### ZCU104 ###
#### Processing System One Time Setup: ####
1. Install ZCU104 SD card image
   - https://ubuntu.com/download/amd-xilinx
1. Configure SW6 to boot from SD1.
   - Mode SW6[4:1]: OFF, OFF, OFF, ON
     ![](zynq_boot_switch.jpg)
1. Install rustup using Unix directions: https://rustup.rs/#
1. apt install libssl-dev

#### Helpful Commands: ####
 - Disable CPU IDLE. Vivado HW Manager access during IDLE causes crashes.
   - `echo 1 > /sys/devices/system/cpu/cpu0/cpuidle/state1/disable`
   - `echo 1 > /sys/devices/system/cpu/cpu1/cpuidle/state1/disable`
   - `echo 1 > /sys/devices/system/cpu/cpu2/cpuidle/state1/disable`
   - `echo 1 > /sys/devices/system/cpu/cpu3/cpuidle/state1/disable`
 - Reduce fan speed.
   - https://support.xilinx.com/s/question/0D52E00006iHuopSAC/zcu104-fan-running-at-max-speed?language=en_US
   - `echo 321 > /sys/class/gpio/export`
   - `echo out > /sys/class/gpio/gpio321/direction`

#### Serial port configuration: ####
Serial port settings for connection over USB.
 - Speed: 115200
 - Data bits: 8
 - Stop bits: 1
 - Parity: None
 - Flow control: None

### FPGA Build Steps: ###
 - Build FPGA image without GUI
    - `vivado -mode batch -source fpga_configuration.tcl -tclargs BUILD=TRUE`
    - Above command creates a bitstream located at: caliptra_build/caliptra_fpga.bin
    - To check the git revision a bitstream was generated with
      - `xxd -s 0x88 -l 8 caliptra_build/caliptra_fpga.bin`
      - Result should be `3001 a001 xxxx xxxx`. 3001 a001 is a command to write the USR_ACCESS register and the rest is the hash.
 - Launch Vivado with GUI
    - `vivado -mode batch -source fpga_configuration.tcl -tclargs GUI=TRUE`
    - Run Synthesis: `launch_runs synth_1`
    - [Optional] Set Up Debug signals on Synthesized Design
    - Run Implementation: `launch_runs impl_1`
    - Generate Bitstream: `write_bitstream -bin_file \tmp\caliptra_fpga`

### AXI Memory Map ###
 - SOC adapter for driving caliptra-top signals
   - 0x80000000 - GPIO Out -> Caliptra
     - `[0] -> cptra_rst_b`
     - `[1] -> cptra_pwrgood`
     - `[5:4] -> device_lifecycle`
     - `[6] -> debug_locked`
     - `[31:24] -> generic_input_wires[7:0] for serial tag`
   - 0x80000008 - GPIO In <- Caliptra
     - `[26] <- cptra_error_fatal`
     - `[27] <- cptra_error_non_fatal`
     - `[28] <- ready_for_fw_push`
     - `[29] <- ready_for_runtime`
     - `[30] <- ready_for_fuses`
   - 0x8000000C - PAUSER
     - `[31:0] -> PAUSER to Caliptra APB`
 - ROM Backdoor
   - `0x82000000 - 0x82007FFF`
 - Caliptra soc register interface
   - `0x90000000`

### Loading and Execution Steps: ###
1. Install FPGA image
    - `sudo fpgautil -b caliptra_fpga_project_bd_wrapper.bin -f Full -n Full`
1. Insert kernel modules for IO access
    - As root:
      - `cd hw-latest/fpga/rom_backdoor`
      - `make`
      - `insmod rom_backdoor.ko`
      - `cd hw-latest/fpga/io_module`
      - `make`
      - `insmod io_module.ko`
      - `chmod 666 /dev/uio4`
1. Set FPGA PLL frequency
    - As root:
      - `echo 20000000 > /sys/bus/platform/drivers/xilinx_fclk/fclk0/set_rate`
1. Execute test targeting fpga_realtime
    - `CPTRA_UIO_NUM=4 cargo test -p caliptra-test --features=fpga_realtime smoke_test`

### JTAG debug [Doc WIP]
JTAG support has been enabled using OpenOCD. The security state debug_locked parameter must be FALSE for JTAG to be accessible.
- `openocd.exe -f ~/openocd_caliptra.txt`

| JTAG Feature                               | Status              |
| ------------------------------------------ | ------------------- |
| RISCV register access                      | Works               |
| VEER MSR                                   | Works               |
| OpenOCD HW breakpoints                     | Works               |
| SW Breakpoints                             | Requires System Bus |
| RISCV Abstract access (ICCM and DCCM only) | Works               |
| System Bus Access                          | Not connected       |
| GDB                                        | Connects but hangs  |