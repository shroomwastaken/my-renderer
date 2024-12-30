#[derive(Debug)]
pub enum RendererError {
	InitError(String),
	DrawError(String),
	FileReadError(String),
}

impl std::fmt::Display for RendererError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			RendererError::InitError(s) => { Ok(f.write_fmt(format_args!("{}", s))?) }
			RendererError::DrawError(s) => { Ok(f.write_fmt(format_args!("{}", s))?) }
			RendererError::FileReadError(s) => { Ok(f.write_fmt(format_args!("unable to read file {}", s))?) }
		}
	}
}

impl std::error::Error for RendererError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			RendererError::InitError(_) => None,
			RendererError::DrawError(_) => None,
			RendererError::FileReadError(_) => None,
		}
	}
}