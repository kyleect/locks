use std::ops::Index;

use arrayvec::ArrayVec;

use crate::error::{OverflowError, Result};
use crate::types::Span;
use crate::vm::value::Value;
use crate::vm::Disassembler;

#[derive(Debug, Default)]
pub struct Chunk {
    pub ops: Vec<u8>,
    pub constants: ArrayVec<Value, 256>,
    pub spans: VecRun<Span>,
}

impl Chunk {
    pub fn write_u8(&mut self, byte: u8, span: &Span) {
        self.ops.push(byte);
        self.spans.push(span.clone());
    }

    /// Writes a constant to the [`Chunk`] and returns its index. If an equal
    /// [`Value`] is already present, then its index is returned instead.
    pub fn write_constant(&mut self, value: Value, span: &Span) -> Result<u8> {
        let idx = match self.constants.iter().position(|&constant| constant == value) {
            Some(idx) => idx,
            None => {
                self.constants
                    .try_push(value)
                    .map_err(|_| (OverflowError::TooManyConstants.into(), span.clone()))?;
                self.constants.len() - 1
            }
        };
        Ok(idx.try_into().expect("constant index overflow"))
    }

    pub fn debug(&self, name: Option<&str>) {
        if let Some(name) = name {
            eprintln!("== {name} ==");
        }

        let result = Disassembler::new(self).disassemble(None);

        eprintln!("{result}");
    }
}

/// Run-length encoded [`Vec`]. Useful for storing data with a lot of contiguous
/// runs of the same value.
#[derive(Debug, Default)]
pub struct VecRun<T> {
    values: Vec<Run<T>>,
}

impl<T: Eq> VecRun<T> {
    fn push(&mut self, value: T) {
        match self.values.last_mut() {
            Some(run) if run.value == value && run.count < u8::MAX => {
                run.count += 1;
            }
            _ => self.values.push(Run { value, count: 1 }),
        };
    }
}

impl<T> Index<usize> for VecRun<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut count = index;
        for run in &self.values {
            match count.checked_sub(run.count as usize) {
                Some(remaining) => count = remaining,
                None => return &run.value,
            }
        }
        panic!("index out of bounds");
    }
}

#[derive(Debug)]
struct Run<T> {
    value: T,
    count: u8,
}
