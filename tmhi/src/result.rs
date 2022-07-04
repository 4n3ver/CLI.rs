use std::num::NonZeroU8;

pub enum Kind {
    Request,
    Status(NonZeroU8),
    Body(String),
    Decode,
}
