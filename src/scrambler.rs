use crate::BitStream;
use rand::Rng;

pub struct Scrambler {
    c_init: u32,
}

impl Scrambler {
    pub fn scramble(&self, bit_stream: &BitStream) -> BitStream {
        // TS 38.211 Sec. 5.2.1
        // Generate scrambling sequence.
        // Pseudo-random sequence defined by a length-31 Gold sequence.
        // https://en.wikipedia.org/wiki/Gold_code
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
}

impl Default for Scrambler {
    fn default() -> Self {
        // TS 38.211 Sec. 7.3.1.1
        // If dataScramblingIdentity is configured in PDSCH-Config, this takes a value from
        // the set {0,1,...,1023}. Otherwise, this is set to the physical cell ID.
        let id = rand::thread_rng().gen_range(0..1024);
        // TODO: Set rnti and q dynamically
        // RNTI = Radio Network Temporary Identifier
        let rnti = 1;
        // Quarter index
        // Codeword number (either 0 or 1)
        let q = 0;
        // c_init = rnti * 2^15 + q * 2^14 + id
        let c_init = (rnti << 15) + (q << 14) + id;
        Self { c_init }
    }
}
