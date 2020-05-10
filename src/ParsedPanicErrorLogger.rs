// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Logs panics.
///
/// Should assume as little as possible about environment sanity.
pub trait ParsedPanicErrorLogger: Send + Sync
{
	/// Log.
	///
	/// May be called more than once from different threads but should only ever be called once or never for each thread
	///
	/// May not be called 'in order', eg:-
	/// * may be called twice, but,
	/// * second call is actually the error from the thread that caused the panic and termination to start.
	fn log(&self, parsed_panic: ParsedPanic);
}
