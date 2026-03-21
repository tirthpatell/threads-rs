use crate::client::Client;
use crate::constants;
use crate::error;
use crate::types::PostId;

impl Client {
    /// Delete a post by ID. Returns the deleted post ID.
    pub async fn delete_post(&self, post_id: &PostId) -> crate::Result<String> {
        if !post_id.is_valid() {
            return Err(error::new_validation_error(
                0,
                constants::ERR_EMPTY_POST_ID,
                "",
                "post_id",
            ));
        }

        let token = self.access_token().await;
        let path = format!("/{}", post_id);
        let resp = self.http_client.delete(&path, &token).await?;

        // Try to parse the response for an id or success field.
        // Fall back to returning the input post_id if unparseable.
        #[derive(serde::Deserialize)]
        struct DeleteResponse {
            #[serde(default)]
            id: Option<String>,
            #[serde(default)]
            _success: Option<bool>,
        }

        if let Ok(del_resp) = serde_json::from_slice::<DeleteResponse>(&resp.body) {
            if let Some(id) = del_resp.id {
                return Ok(id);
            }
        }

        Ok(post_id.to_string())
    }
}
