use std::fmt::{Display, self};

/// All possible "trap" codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TrapCode
{
    /// The trap has no associated trap code.
    Undefined = -1,

    /// The trap was the result of exhausting the available stack space.
    StackOverflow = 0,

    /// The trap was the result of an out-of-bounds memory access.
    MemoryOutOfBounds = 1,

    /// The trap was the result of a wasm atomic operation that was presented with a misaligned linear-memory address.
    HeapMisaligned = 2,

    /// The trap was the result of an out-of-bounds access to a table.
    TableOutOfBounds = 3,

    /// The trap was the result of an indirect call to a null table entry.
    IndirectCallToNull = 4,

    /// The trap was the result of a signature mismatch on indirect call.
    BadSignature = 5,

    /// The trap was the result of an integer arithmetic operation that overflowed.
    IntegerOverflow = 6,

    /// The trap was the result of an integer division by zero.
    IntegerDivisionByZero = 7,

    /// The trap was the result of a failed float-to-int conversion.
    BadConversionToInteger = 8,

    /// The trap was the result of executing the `unreachable` instruction.
    Unreachable = 9,

    /// The trap was the result of interrupting execution.
    Interrupt = 10,

    /// The trap was the result of running out of the configured fuel amount.
    OutOfFuel = 11,
}

impl Display for TrapCode
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        write!(formatter, "{}", self)
    }
}