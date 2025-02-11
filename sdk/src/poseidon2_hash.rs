#[cfg(target_os = "zkvm")]
use crate::syscall_poseidon2_permute;

/// A stateful hasher for Poseidon2.
#[derive(Default)]
pub struct Poseidon2<const OUT: usize> {
    state: [u32; 16],    // Poseidon2 works with a 16-element state.
    buffer_count: usize, // Number of elements currently buffered.
}

impl<const OUT: usize> Poseidon2<OUT> {
    /// Creates a new Poseidon2 hasher with an empty state.
    pub fn new() -> Self {
        Self {
            state: [0; 16],
            buffer_count: 0,
        }
    }

    /// Updates the hasher state with a new input element.
    pub fn update(&mut self, input: u32) {
        // Buffer the input directly into the state.
        self.state[self.buffer_count] += input;
        self.buffer_count += 1;

        // If the buffer is full (15 elements), absorb and reset the buffer count.
        if self.buffer_count == 15 {
            self.permute();
            self.buffer_count = 0;
            // println!(">> permute round state: {:?}", self.state);
        }
    }

    /// Finalizes the hashing process and returns the resulting hash.
    pub fn finalize(mut self) -> [u32; OUT] {
        // Pad remaining elements.
        if self.buffer_count > 0 {
            self.state[self.buffer_count] += 1; // Padding with `1`.
        } else {
            self.state[0] += 1; // If empty, pad the first element.
        }
        self.permute(); // Apply the final permutation.

        self.state[0..OUT].try_into().unwrap()
    }

    /// Computes the Poseidon2 permutation on the state.
    fn permute(&mut self) {
        #[allow(unused_mut)]
        let mut ret = [0_u32; 16];
        #[cfg(target_os = "zkvm")]
        unsafe {
            syscall_poseidon2_permute(&self.state as *const _, &mut ret as *mut _);
        }
        self.state = ret;
    }

    /// A convenience function to hash two elements.
    pub fn hash_two(x: u32, y: u32) -> [u32; OUT] {
        let mut state = [0_u32; 16];
        state[0] += x;
        state[1] += y;

        #[allow(unused_mut)]
        let mut ret = [0_u32; 16];
        #[cfg(target_os = "zkvm")]
        unsafe {
            syscall_poseidon2_permute(&state as *const _, &mut ret as *mut _);
        }
        ret[0..OUT].try_into().unwrap()
    }

    /// A convenience function to hash a single element.
    pub fn hash_single(x: u32) -> [u32; OUT] {
        let mut state = [0_u32; 16];
        state[0] += x;

        #[allow(unused_mut)]
        let mut ret = [0_u32; 16];
        #[cfg(target_os = "zkvm")]
        unsafe {
            syscall_poseidon2_permute(&state as *const _, &mut ret as *mut _);
        }
        ret[0..OUT].try_into().unwrap()
    }

    /// A convenience function to hash multiple elements.
    pub fn hash(inputs: &[u32]) -> [u32; OUT] {
        let mut hasher = Poseidon2::new();
        for &input in inputs {
            hasher.update(input);
        }
        hasher.finalize()
    }
}
