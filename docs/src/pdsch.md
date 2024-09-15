# PDSCH Transport Process


1. Transport block CRC attachment
2. LDPC base graph selection
3. Code block segmentation And Code Block CRC Attachment
4. Channel Coding
5. Rate Matching
6. Code Block Concatenation
7. Scrambling
8. Modulation
9. Layer Mapping
10. Antenna Port Mapping
11. Mapping to VRB
12. Mapping from virtual to physical resource blocks

## Transport block CRC attachment
## LDPC base graph selection
## Code block segmentation And Code Block CRC Attachment
## Channel Coding
## Rate Matching
## Code Block Concatenation
## Scrambling

Scrambling introduces randomness in the transmitted data to ensure uniform power distribution, interference management, data privacy and channel estimation. The scrambling and descrambling process is performed by the transmitter and receiver respecively.

```rust
pub fn scramble(&self, bit_stream: &BitStream) -> BitStream {
    // TS 38.211 Sec. 5.2.1
    // Pseudo-random sequence defined by a length-31 Gold sequence.
    // https://en.wikipedia.org/wiki/Gold_code

    // Generate scrambling sequence.
    let length = bit_stream.len();
    let n_c = 1600;
    let seq_length = length + n_c;
    let mut x1 = vec![0u8; seq_length + 31];
    let mut x2 = vec![0u8; seq_length + 31];
    let mut c = vec![0u8; length];

    x1[1..31].fill(1);

    for n in 0..31 {
        x2[n] = ((self.c_init >> n) & 0x1) as u8;
    }

    for n in 0..seq_length {
        x1[n + 31] = x1[n + 3] ^ x1[n];
    }

    for n in 0..seq_length {
        x2[n + 31] = x2[n + 3] ^ x2[n + 2] ^ x2[n + 1] ^ x2[n];
    }

    for n in 0..length {
        c[n] = x1[n + n_c] ^ x2[n + n_c];
    }

    // Scramble bit stream.
    bit_stream
        .iter()
        .zip(c)
        .map(|(bits, scramble)| bits ^ scramble)
        .collect()
}
```


## Modulation
The modulation step converts the binary data stream into complex symbols that are suitable for wireless transmission. There are multiple different modulation schemes. E.g.,
  - QPSK: QPSK modulates 2 bits per symbol.
  - 16QAM: 16QAM modulates 4 bits per symbol.
  - 64QAM: 64QAM modulates 6 bits per symbol.
  - 256QAM: 256QAM modulates 8 bits per symbol.

The more bits that are modulated per symbol the higher the data rate. However, it comes at a cost of reduced robustness. E.g., 256QAM is more prone to noise and interference than QPSK, but can offer much greater data rates.

### QPSK
The formula for transforming a bit stream defined as `b(i)` where `i` represents the ith bit to QPSK symbols `d(i)` looks like:
$$
d(i)=\frac{1}{√2}[(1−2b(2i))+j(1−2b(2i+1))]
$$


Modelling the `modulate` routine for QPSK in Rust may look something like the following:

```rust
fn modulate(bit_stream: &BitStream) -> SymbolStream {
    // d(i)=1√2[(1−2b(2i))+j(1−2b(2i+1))]
    //   (imag)
    //     ^
    //  10 | 00
    //  ---|---> (real)
    //  11 | 01
    bit_stream
       .iter()
       .map(|bits| {
           let re = if bits & 0b10 == 0 { 1.0 } else { -1.0 };
           let im = if bits & 0b01 == 0 { 1.0 } else { -1.0 };
           Symbol::new(Complex::new(re, im))
       })
       .collect()
}
```

while the `demodulate` routine looks something like:
```rust
fn demodulate(symbols: &SymbolStream) -> BitStream {
    symbols
        .iter()
        .map(|symbol| {
            let mut bits = 0u8;
            bits |= if symbol.complex.re <= 0.0 { 0b10 } else { 0 };
            bits |= if symbol.complex.im <= 0.0 { 0b01 } else { 0 };
            bits
        })
        .collect()
}
```


$$
r(n)=\frac{1}{√2}(1−2⋅c(2n))+j\frac{1}{√2}(1−2⋅c(2n+1)).
$$


[1] [https://www.nrexplained.com/modulation](https://www.nrexplained.com/modulation)

## Layer Mapping
## Antenna Port Mapping
## Mapping to VRB
## Mapping from virtual to physical resource blocks



# References
[https://www.sharetechnote.com/html/5G/5G_PDSCH.html#PDSCH_Transport_Process](https://www.sharetechnote.com/html/5G/5G_PDSCH.html#PDSCH_Transport_Process)