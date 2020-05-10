// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// What caused the panic?
#[inline(always)]
pub fn panic_payload_to_cause<'panic_info>(panic_payload: &'panic_info (dyn Any + 'static + Send)) -> &'panic_info str
{
	if panic_payload.is::<String>()
	{
		panic_payload.downcast_ref::<String>().unwrap()
	}
	else if panic_payload.is::<&str>()
	{
		panic_payload.downcast_ref::<&str>().unwrap()
	}
	else
	{
		"(unknown cause)"
	}
}
