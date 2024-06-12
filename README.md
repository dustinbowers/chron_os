# Chron OS
Just a kernel written in Rust ...for now

Emulation can be done with QEMU: https://www.qemu.org/

**Running:** `cargo run`  
**Testing:**` cargo test`

# How it works
## VGA Text Buffer (memory-mapped I/O)
The VGA text buffer supports reads/writes and is memory-mapped to `0x8000`. It's typically 80 columns wide and 25 rows high.  
  
Each array entry is 2-bytes wide in this format:

| Bit(s) | Value            |
|--------|------------------|
| 0-7    | ASCII code point |
| 8-11   | Foreground color |
| 12-14  | Background color |
| 15     | Blink            |

### ASCII code points
See: [code page 437](https://en.wikipedia.org/wiki/Code_page_437)

### Color representation
| Number | Color | Number + Bright Bit | Bright Color |
|---|---|---|---|
|0x0|Black|0x8|Dark Gray|
|0x1|Blue|0x9|Light Blue|
|0x2|Gree|0xa|Light Green|
|0x3|Cyan|0xb|Light Cyan|
|0x4|Red|0xc|Light Red|
|0x5|Magenta|0xd|Pink|
|0x6|Brown|0xe|Yellow|
|0x7|Light Gray|0xf|White||


## Port-mapped I/O
With QEMU, we can set an [ISA](https://en.wikipedia.org/wiki/Industry_Standard_Architecture) device used for debug exits. We set the `isa-debug-exit` device to `0xf4` which is not [commonly used](https://wiki.osdev.org/I/O_Ports#The_list). This is set in Cargo.toml using the test-args:  
> `"-device", "isa-debug-exit,iobase=0xf4,iosize=0x04", "-serial", "stdio",` 

## Printing to console
QEMU allows piping serial to stdin. In the kernel, we can use [16550 UART](https://en.wikipedia.org/wiki/16550_UART) to send serial data. The first serial interface address is at `0x03F8`. Using more arguments to QEMU's `test-args` (in Cargo.toml) we can redirect serial to stdout like this:  
> `"-serial", "stdio"`

