// This file is part of terminate. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/terminate/master/COPYRIGHT. No part of terminate, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of terminate. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/terminate/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]


//! #terminate
//! 
//! Is a rust crate to abstract logic to terminate threads, coroutines and the like.


use ::std::any::Any;
use ::std::panic::PanicInfo;
use ::std::sync::Arc;
use ::std::sync::atomic::AtomicBool;
use ::std::sync::atomic::Ordering::Relaxed;


include!("SimpleTerminate.rs");
include!("Terminate.rs");
