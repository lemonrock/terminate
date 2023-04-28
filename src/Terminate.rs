// This file is part of terminate. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/terminate/master/COPYRIGHT. No part of terminate, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of terminate. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/terminate/master/COPYRIGHT.


/// Abstracts the need to terminate a loop or application and to begin and check termination.
pub trait Terminate: Send + Sync
{
	/// Begin termination.
	fn begin_termination(&self);

	/// Begin termination (due to a panic).
	///
	/// Can be called more than once, but should only be called once per thread.
	#[inline(always)]
	fn begin_termination_due_to_panic(&self, panic_info: &PanicInfo)
	{
		self.begin_termination_due_to_irrecoverable_error(panic_info.payload(), panic_info.location())
	}

	/// Begin termination (due to an irrecoverable error).
	///
	/// Can be called more than once, but should only be called once per thread.
	///
	/// `irrecoverable_error` is the same type as `PanicInfo.payload`.
	/// `location` can be `None`.
	fn begin_termination_due_to_irrecoverable_error(&self, irrecoverable_error: &dyn Any, location: Option<&Location>);

	/// Should finish.
	fn should_finish(&self) -> bool;

	/// Should continue (opposite of `should_finish()`).
	fn should_continue(&self) -> bool
	{
		!self.should_finish()
	}

	/// Check after termination.
	fn terminated_due_to_panic_or_irrecoverable_error(&self) -> bool;
}
