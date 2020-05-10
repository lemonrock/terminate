// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A simple terminate that does no logging of errors.
#[derive(Debug)]
pub struct SimpleTerminate<PPEL: ParsedPanicErrorLogger>
{
	lock: AtomicU64,
	error_logger: PPEL,
}

impl<PPEL: ParsedPanicErrorLogger> Terminate for SimpleTerminate<PPEL>
{
	/// Begin termination.
	#[inline(always)]
	fn begin_termination(&self)
	{
		self.compare_and_swap(Self::NormalTerminate)
	}

	#[inline(always)]
	fn begin_termination_due_to_irrecoverable_error(&self, panic_payload: &(dyn Any + Send), location: Option<&Location>)
	{
		self.compare_and_swap(Self::PanicTerminate);
		let parsed_panic = ParsedPanic::parse(panic_payload, location);
		self.error_logger.log(parsed_panic)
	}

	#[inline(always)]
	fn should_finish(&self) -> bool
	{
		self.load() != Self::Continue
	}

	#[inline(always)]
	fn terminated_due_to_panic_or_irrecoverable_error(&self) -> bool
	{
		let value = self.load();
		debug_assert_ne!(value, Self::Continue, "should not be checking this unless `should_finish()` was `true`");
		value == Self::PanicTerminate
	}
}

impl<PPEL: ParsedPanicErrorLogger> SimpleTerminate<PPEL>
{
	const Continue: u64 = 0x00;

	const NormalTerminate: u64 = 0x01;

	const PanicTerminate: u64 = 0x02;

	/// New instance.
	#[inline(always)]
	pub fn new(error_logger: PPEL) -> Arc<Self>
	{
		Arc::new
		(
			Self
			{
				lock: AtomicU64::new(Self::Continue),
				error_logger,
			}
		)
	}

	#[inline(always)]
	fn compare_and_swap(&self, termination_reason: u64)
	{
		self.lock.compare_and_swap(Self::Continue, termination_reason, AcqRel);
	}

	#[inline(always)]
	fn load(&self) -> u64
	{
		self.lock.load(Acquire)
	}
}
