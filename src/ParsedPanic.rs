// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
pub struct ParsedPanic<'panic_info>
{
	thread_causing_panic: Thread,

	pub source_file: Cow<'panic_info, str>,

	pub line_number: u32,

	pub column_number: u32,

	pub cause: &'panic_info str,

	pub backtrace: Cow<'static, str>,
}

impl<'panic_info> ParsedPanic<'panic_info>
{
	/// Thread name.
	#[inline(always)]
	pub fn thread_name(&self) -> &str
	{
		self.thread_causing_panic.name().unwrap_or("(unknown thread)")
	}

	/// Thread identifier.
	#[inline(always)]
	pub fn thread_id(&self) -> ThreadId
	{
		self.thread_causing_panic.id()
	}

	#[inline(always)]
	fn parse(panic_payload: &'panic_info dyn Any, location: Option<&'panic_info Location<'panic_info>>) -> Self
	{
		let thread_causing_panic = thread::current();

		let (source_file, line_number, column_number) = match location
		{
			None => (Cow::from("(unknown source file)"), 0, 0),
			Some(location) => (Cow::from(location.file()), location.line(), location.column())
		};

		let cause = panic_payload_to_cause(panic_payload);

		let backtrace = Backtrace::capture();
		let backtrace = if backtrace.status() == BacktraceStatus::Captured
		{
			Cow::from(format!("{}", backtrace).replace('\n', "|"))
		}
		else
		{
			Cow::from("(missing backtrace)")
		};

		Self
		{
			thread_causing_panic,
			source_file,
			line_number,
			column_number,
			cause,
			backtrace
		}
	}
}
