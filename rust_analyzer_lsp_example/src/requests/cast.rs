use lsp_server::{ExtractError, Request, RequestId};

// Convert to a rust struct/enum
//
pub fn cast_request<R>(
  req: Request
) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
  R: lsp_types::request::Request,
  R::Params: serde::de::DeserializeOwned,
{
  req.extract(R::METHOD)
}
