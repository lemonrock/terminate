// This file is part of terminate. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/terminate/master/COPYRIGHT. No part of terminate, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of terminate. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/terminate/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(backtrace)]


//! #terminate
//! 
//! Is a rust crate to abstract logic to terminate threads, coroutines and the like.


use std::any::Any;
use std::panic::PanicInfo;
use std::panic::Location;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::AcqRel;
use std::sync::atomic::Ordering::Acquire;
use std::thread;
use std::backtrace::Backtrace;
use std::backtrace::BacktraceStatus;
use std::thread::Thread;
use std::thread::ThreadId;
use std::borrow::Cow;


include!("panic_payload_to_cause.rs");
include!("ParsedPanic.rs");
include!("ParsedPanicErrorLogger.rs");
include!("SimpleTerminate.rs");
include!("Terminate.rs");
