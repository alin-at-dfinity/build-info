use anyhow::Result;
use format_buf::format;

use std::any::Any;

use build_info_common::CompilerInfo;

use super::{as_arguments_0, as_field_name, FormatSpecifier, Type, Value};

impl Value for CompilerInfo {
	fn call(&self, func: &str, args: &[&dyn Value]) -> Result<Box<dyn Value>> {
		match func {
			"!field" => match as_field_name(args) {
				"version" => Ok(Box::new(self.version.clone())),
				"commit_id" => Ok(Box::new(self.commit_id.clone())),
				"commit_date" => Ok(Box::new(self.commit_date)),
				"channel" => Ok(Box::new(self.channel)),
				"host_triple" => Ok(Box::new(self.host_triple.clone())),
				"target_triple" => Ok(Box::new(self.target_triple.clone())),
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
		Type::CompilerInfo
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
