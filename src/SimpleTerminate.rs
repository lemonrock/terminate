// This file is part of terminate. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/terminate/master/COPYRIGHT. No part of terminate, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of terminate. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/terminate/master/COPYRIGHT.


/// Simple implementation of Terminate.
#[derive(Default, Debug)]
pub struct SimpleTerminate
{
	terminate: AtomicBool,
	terminated_due_to_panic_or_irrecoverable_error: AtomicBool,
}

impl Terminate for SimpleTerminate
{
	#[inline(always)]
	fn begin_termination(&self)
	{
		self.terminate.store(true, Relaxed)
	}

	#[inline(always)]
	fn begin_termination_due_to_panic(&self, _panic_info: &PanicInfo)
	{
		self.set_terminated_due_to_panic_or_irrecoverable_error();
		self.begin_termination()
	}

	#[inline(always)]
	fn begin_termination_due_to_irrecoverable_error(&self, _irrecoverable_error: &(dyn Any + Send))
	{
		self.set_terminated_due_to_panic_or_irrecoverable_error();
		self.begin_termination()
	}

	#[inline(always)]
	fn should_finish(&self) -> bool
	{
		self.terminate.load(Relaxed)
	}

	#[inline(always)]
	fn terminated_due_to_panic_or_irrecoverable_error(&self) -> bool
	{
		self.terminated_due_to_panic_or_irrecoverable_error.load(Relaxed)
	}
}

impl SimpleTerminate
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new() -> Arc<Self>
	{
		Arc::new
		(
			Self
			{
				terminate: AtomicBool::new(false),
				terminated_due_to_panic_or_irrecoverable_error: AtomicBool::new(false),
			}
		)
	}

	#[inline(always)]
	fn set_terminated_due_to_panic_or_irrecoverable_error(&self)
	{
		self.terminated_due_to_panic_or_irrecoverable_error.store(true, Relaxed);
	}
}
