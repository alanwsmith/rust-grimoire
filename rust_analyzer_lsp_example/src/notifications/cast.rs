use lsp_server::{ExtractError, Notification};

pub fn cast_notify<N>(
  notif: Notification
) -> Result<N::Params, ExtractError<Notification>>
where
  N: lsp_types::notification::Notification,
  N::Params: serde::de::DeserializeOwned,
{
  notif.extract(N::METHOD)
}
