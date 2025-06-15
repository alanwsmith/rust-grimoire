use lsp_server::{ExtractError, Request, RequestId};

pub fn cast_notify<N>(
  notif: lsp_server::Notification
) -> Result<
  N::Params,
  ExtractError<lsp_server::Notification>,
>
where
  N: lsp_types::notification::Notification,
  N::Params: serde::de::DeserializeOwned,
{
  notif.extract(N::METHOD)
}

fn cast_request<R>(
  req: Request
) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
  R: lsp_types::request::Request,
  R::Params: serde::de::DeserializeOwned,
{
  req.extract(R::METHOD)
}
