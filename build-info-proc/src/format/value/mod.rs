use anyhow::{anyhow, Result};
use format_buf::format;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

use std::any::{type_name, Any};
use std::fmt::Debug;

use super::Type;

mod bool;
mod int;
mod option;
mod string;
mod vec;

mod chrono;
mod semver;

mod build_info;
mod compiler_channel;
mod compiler_info;
mod crate_info;
mod git_info;
mod version_control;

pub(crate) trait Value: Debug {
	fn call_base(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			OP_FIELD_ACCESS => {
				let field = as_field_name(args);
				Err(anyhow!(
					"The field {} does not exist for objects of type {}",
					field,
					self.get_type()
				))
			}
			OP_TUPLE_INDEX => Err(anyhow!("Type {} cannot be tuple-indexed", self.get_type())),
			OP_ARRAY_INDEX => Err(anyhow!("Type {} cannot be indexed", self.get_type())),
			_ => Err(anyhow!(
				"Function {} cannot be called with arguments {:#?} on objects of type {}",
				func,
				args,
				self.get_type()
			)),
		}
	}

	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		self.call_base(func, args)
	}

	fn get_type(&self) -> Type;

	fn as_any(&self) -> &dyn Any;

	fn format(&self, buffer: &mut String, spec: FormatSpecifier);
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum FormatSpecifier {
	Default,
	Debug,
	DebugAlt,
}

pub(crate) const OP_FIELD_ACCESS: &str = "!field";
pub(crate) const OP_TUPLE_INDEX: &str = "!tuple_index";
pub(crate) const OP_ARRAY_INDEX: &str = "!array_index";

fn as_field_name<'a>(args: &[&'a dyn Value]) -> &'a str {
	assert!(
		args.len() == 1,
		"Accessing a field must have exactly one operand (the field name)"
	);

	args[0]
		.as_any()
		.downcast_ref::<String>()
		.expect("The field name must be a string when accessing a field.")
}

fn as_index(args: &[&dyn Value]) -> usize {
	assert!(
		args.len() == 1,
		"Accessing a field must have exactly one operand (the field name)"
	);

	args[0]
		.as_any()
		.downcast_ref::<BigInt>()
		.expect("The array index must be an integer.")
		.to_usize()
		.expect("The array index does not fit into the type usize.")
}

fn as_arguments_0(args: &[&dyn Value]) -> Result<()> {
	if args.is_empty() {
		Ok(())
	} else {
		Err(anyhow!("Wrong number of arguments (should be 0)"))
	}
}

#[allow(dead_code)]
fn as_arguments_1<'a, T1: 'static>(args: &[&'a dyn Value]) -> Result<(&'a T1,)> {
	if args.len() != 1 {
		return Err(anyhow!("Wrong number of arguments (should be 1)"));
	}

	Ok((args[0]
		.as_any()
		.downcast_ref::<T1>()
		.ok_or_else(|| anyhow!("Argument #1 should have type {}", type_name::<T1>()))?,))
}

#[allow(dead_code)]
fn as_arguments_2<'a, T1: 'static, T2: 'static>(args: &[&'a dyn Value]) -> Result<(&'a T1, &'a T2)> {
	if args.len() != 2 {
		return Err(anyhow!("Wrong number of arguments (should be 1)"));
	}

	Ok((
		args[0]
			.as_any()
			.downcast_ref::<T1>()
			.ok_or_else(|| anyhow!("Argument #1 should have type {}", type_name::<T1>()))?,
		args[1]
			.as_any()
			.downcast_ref::<T2>()
			.ok_or_else(|| anyhow!("Argument #2 should have type {}", type_name::<T1>()))?,
	))
}
