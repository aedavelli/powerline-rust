use std::{fs, marker::PhantomData};

use super::Module;
use crate::{terminal::Color, Segment, R};

pub struct ReadOnly<S>(PhantomData<S>);

pub trait ReadOnlyScheme {
	const READONLY_FG: Color;
	const READONLY_BG: Color;
	const READONLY_SYMBOL: &'static str = "";
}
impl<S: ReadOnlyScheme> ReadOnly<S> {
	pub fn new() -> ReadOnly<S> {
		ReadOnly(PhantomData)
	}
}

impl<S: ReadOnlyScheme> Module for ReadOnly<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		let readonly = fs::metadata("./")?.permissions().readonly();

		if readonly {
			segments.push(Segment::simple(
				format!(" {} ", S::READONLY_SYMBOL),
				S::READONLY_FG,
				S::READONLY_BG,
			));
		}

		Ok(())
	}
}
