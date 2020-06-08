use anyhow::Result;
use format_buf::format;

use std::any::Any;

use build_info_common::GitInfo;

use super::{as_arguments_0, as_field_name, FormatSpecifier, Type, Value};

impl Value for GitInfo {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"!field" => match as_field_name(args) {
				"commit_id" => Ok(Box::new(self.commit_id.clone())),
				"commit_short_id" => Ok(Box::new(self.commit_short_id.clone())),
				"commit_timestamp" => Ok(Box::new(self.commit_timestamp)),
				"dirty" => Ok(Box::new(self.dirty)),
				"branch" => Ok(Box::new(self.branch.clone())),
				"tags" => Ok(Box::new(self.tags.clone())),
				_ => self.call_base(func, args),
			},
			"to_string" => {
				as_arguments_0(args)?;
				Ok(Box::new(self.to_string()))
			}
			_ => self.call_base(func, args),
		}
	}

	fn get_type(&self) -> Type {
		Type::GitInfo
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn format(&self, buffer: &mut String, spec: FormatSpecifier) {
		match spec {
			FormatSpecifier::Default => format!(buffer, "{}", self),
			FormatSpecifier::Debug => format!(buffer, "{:?}", self),
			FormatSpecifier::DebugAlt => format!(buffer, "{:#?}", self),
		}
	}
}
