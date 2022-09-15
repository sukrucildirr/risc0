// Copyright 2022 Risc0, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::{cell::UnsafeCell, mem::MaybeUninit, slice};

use risc0_zkp::core::sha::Digest;
use risc0_zkvm::serde::{Deserializer, Serializer, Slice};
// Re-export for easy use by user programs.
#[cfg(target_os = "zkvm")]
pub use risc0_zkvm_platform::rt::host_io::host_sendrecv;
use risc0_zkvm_platform::{
    io::{
        IoDescriptor, GPIO_COMMIT, GPIO_CYCLECOUNT, GPIO_LOG, SENDRECV_CHANNEL_INITIAL_INPUT,
        SENDRECV_CHANNEL_STDOUT,
    },
    memory,
    rt::host_io::host_recv,
    WORD_SIZE,
};
use serde::{Deserialize, Serialize};

use crate::{align_up, memory_barrier, sha};

#[cfg(not(target_os = "zkvm"))]
// Bazel really wants to compile this file for the host too, so provide a stub.
/// This is a stub version of `risc0_zkvm_platform::rt::host_sendrecv`,
/// re-exported for easy access through the SDK.
pub fn host_sendrecv(_channel: u32, _buf: &[u8]) -> (&'static [u32], usize) {
    unimplemented!()
}

struct Env {
    output: Serializer<Slice<'static>>,
    commit: Serializer<Slice<'static>>,
    commit_len: usize,
    initial_input_reader: Option<Reader>,
}

struct Once<T> {
    data: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Send + Sync> Sync for Once<T> {}

/// Reads and deserializes objects from a section of memory.
pub struct Reader(Deserializer<'static>);

impl Reader {
    /// Reads private data from the host.
    pub fn read<T: Deserialize<'static>>(&mut self) -> T {
        T::deserialize(&mut self.0).unwrap()
    }
}

impl<T> Once<T> {
    const fn new() -> Self {
        Once {
            data: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    fn init(&self, value: T) {
        unsafe { &mut *(self.data.get()) }.write(value);
    }

    fn get(&self) -> &mut T {
        unsafe {
            self.data
                .get()
                .as_mut()
                .unwrap_unchecked()
                .assume_init_mut()
        }
    }
}

static ENV: Once<Env> = Once::new();

pub(crate) fn init() {
    ENV.init(Env::new());
}

pub(crate) fn finalize(result: *mut usize) {
    ENV.get().finalize(result);
}

/// Exchanges data with the host, returning data from the host
/// as a slice of bytes.
/// See `env::write` for details on passing structured data to the
/// host.
pub fn send_recv(channel: u32, buf: &[u8]) -> &'static [u8] {
    ENV.get().send_recv(channel, buf)
}

/// Exchanges data with the host, returning the data from the host as
/// a slice of words and the length in bytes.
pub fn send_recv_as_u32(channel: u32, buf: &[u8]) -> (&'static [u32], usize) {
    ENV.get().send_recv_as_u32(channel, buf)
}

/// Reads private data from the host.
///
/// # Examples
/// Values are read in the order in which they are written by the host,
/// as in a queue. In the following example, `first_value` and `second_value`
/// have been shared consecutively by the host via
/// `prover.add_input_u32_slice()` or `prover.add_input_u8_slice()`.
///
/// ```rust, ignore
/// let first_value: u64 = env::read();
/// let second_value: u64 = env::read();
/// ```
/// For ease and clarity, we recommend sharing multiple values between guest and
/// host as a struct. In this example, we read in details about an overdue
/// library book.
/// ```rust, ignore
/// #[derive(Serialize, Deserialize, Debug)]
/// struct LibraryBookDetails {
///     book_id: u64,
///     overdue: bool
/// }
///
/// let book_info: LibraryBookDetails = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();
/// ```
pub fn read<T: Deserialize<'static>>() -> T {
    ENV.get().read()
}

/// Writes private data to the host.
///
/// # Arguments
///
/// * `data` - serialized data to be made available in host-readable memory.
/// # Example
/// In this example, the value `42` is written and is then
/// accessible to the host via `prover.get_output()`.
/// ```rust, ignore
/// let integer_to_share: u32 = 42;
/// env::write(&integer_to_share);
/// ```
pub fn write<T: Serialize>(data: &T) {
    ENV.get().write(data);
}

/// Commits public data to the journal.
///
/// # Examples
/// In this example, we want to publicly share the results of a private
/// computation, so we commit the value to the journal.
/// ```rust, ignore
/// env::commit(&some_result);
/// ```
/// When committing values to the journal, keep in mind that journal contents
/// must be deserialized from a single vector. If multiple values are to be
/// committed, consider creating a commitment data struct.
/// In our [digital signature example](https://github.com/risc0/risc0-rust-examples/tree/main/digital-signature),
/// we commit message and signature together.
/// ```rust, ignore
/// pub struct SigningRequest {
///     pub passphrase: Passphrase,
///     pub msg: Message,
/// }
///
/// env::commit(&SignMessageCommit {
///     identity: *sha::digest(&request.passphrase.pass),
///     msg: request.msg,
/// });
/// ```
pub fn commit<T: Serialize>(data: &T) {
    ENV.get().commit(data);
}

/// Returns the number of processor cycles that have occurred since the guest
/// began.
///
/// # Examples
/// ```rust, ignore
/// let count = get_cycle_count();
/// ```
/// This function can be used to note how many cycles have elapsed during a
/// guest operation:
/// ```
/// let count1 = get_cycle_count();
/// doSomething();
/// let count2 = get_cycle_count();
/// let cycles_elapsed = count2 - count1;
/// ```
pub fn get_cycle_count() -> usize {
    ENV.get().get_cycle_count()
}

/// Print a message to the debug console.
///
/// # Example
/// ```
/// env::log("This is an example log message");
/// ```
pub fn log(msg: &str) {
    // TODO: format! is expensive, replace with a better solution.
    let msg = alloc_crate::format!("{}\0", msg);
    let ptr = msg.as_ptr();
    memory_barrier(ptr);
    unsafe { GPIO_LOG.as_ptr().write_volatile(ptr) };
}

impl Env {
    fn new() -> Self {
        Env {
            commit: Serializer::new(Slice::new(unsafe {
                slice::from_raw_parts_mut(memory::COMMIT.start() as _, memory::COMMIT.len_words())
            })),
            output: Serializer::new(Slice::new(unsafe {
                slice::from_raw_parts_mut(memory::OUTPUT.start() as _, memory::OUTPUT.len_words())
            })),

            commit_len: 0,
            initial_input_reader: None,
        }
    }

    pub fn send_recv_as_u32(&mut self, channel: u32, buf: &[u8]) -> (&'static [u32], usize) {
        host_sendrecv(channel, buf)
    }

    pub fn send_recv(&mut self, channel: u32, buf: &[u8]) -> &'static [u8] {
        let (data, bytes) = self.send_recv_as_u32(channel, buf);
        &bytemuck::cast_slice(data)[..bytes]
    }

    fn initial_input(&mut self) -> &mut Reader {
        if !self.initial_input_reader.is_some() {
            let (words, _) = self.send_recv_as_u32(SENDRECV_CHANNEL_INITIAL_INPUT, &[]);
            self.initial_input_reader = Some(Reader(Deserializer::new(words)))
        }
        self.initial_input_reader.as_mut().unwrap()
    }

    pub fn read<T: Deserialize<'static>>(&mut self) -> T {
        self.initial_input().read()
    }

    fn write<T: Serialize>(&mut self, data: &T) {
        data.serialize(&mut self.output).unwrap();
        let buf = self.output.release().unwrap();
        self.send_recv(SENDRECV_CHANNEL_STDOUT, bytemuck::cast_slice(buf));
    }

    fn commit<T: Serialize>(&mut self, data: &T) {
        data.serialize(&mut self.commit).unwrap();
        let buf = self.commit.release().unwrap();
        self.commit_len += buf.len();
        // Copy to stdout
        self.send_recv(SENDRECV_CHANNEL_STDOUT, bytemuck::cast_slice(buf));
    }

    fn finalize(&mut self, result: *mut usize) {
        let len_words = self.commit_len;
        let len_bytes = len_words * WORD_SIZE;
        let slice: &mut [u32] = unsafe {
            slice::from_raw_parts_mut(memory::COMMIT.start() as _, memory::COMMIT.len_words())
        };
        // Write the full data out to the host
        unsafe {
            let desc = IoDescriptor {
                size: len_bytes as u32,
                addr: slice.as_ptr() as u32,
            };
            let ptr: *const IoDescriptor = &desc;
            memory_barrier(ptr);
            GPIO_COMMIT.as_ptr().write_volatile(&desc);
        }

        // If the total proof message is small (<= 32 bytes), return it directly
        // from the proof. Otherwise, SHA it and return the hash.
        if len_words <= 8 {
            for i in 0..len_words {
                unsafe {
                    result
                        .add(i)
                        .write_volatile(*slice.get_unchecked(i) as usize)
                };
            }
            for i in len_words..8 {
                unsafe { result.add(i).write_volatile(0) };
            }
        } else {
            let cap = sha::compute_capacity_needed(len_bytes);
            let mut slice = &mut slice[..cap];
            sha::add_trailer(&mut slice, len_bytes, sha::MemoryType::WOM);

            let digest = result as *mut Digest;
            // SAFETY: result is a pointer to the output digest.
            unsafe {
                sha::raw_digest_to(&slice, digest);
            }
        }
        unsafe {
            result.add(8).write_volatile(len_bytes);
            memory_barrier(result);
        };
        sha::finalize();
    }

    /// Gets the current count of instruction cycles.
    fn get_cycle_count(&self) -> usize {
        unsafe { GPIO_CYCLECOUNT.as_ptr().write_volatile(0) }
        match host_recv(1) {
            &[nbytes] => nbytes as usize,
            _ => unreachable!(),
        }
    }
}
