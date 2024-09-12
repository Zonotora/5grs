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
impl Modulate for Qpsk {
    fn modulate(bit_stream: &BitStream) -> SymbolStream {
        // d(i)=1√2[(1−2b(2i))+j(1−2b(2i+1))]
        //   (imag)
        //     ^
        //  10 | 00
        //  ---|---> (real)
        //  11 | 01
        let mut symbols: SymbolStream = vec![];
        let sqrt: f32 = 1.0 / (2.0_f32).sqrt();
        let re_map = [sqrt, -sqrt, -sqrt, sqrt];
        let im_map = [sqrt, sqrt, -sqrt, -sqrt];
        for bits in bit_stream {
            let re = re_map[*bits as usize];
            let im = im_map[*bits as usize];
            let complex = Complex::new(re, im);
            let symbol = Symbol::new(complex);
            symbols.push(symbol)
        }
        symbols
    }
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