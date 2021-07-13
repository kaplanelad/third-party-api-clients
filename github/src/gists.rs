use anyhow::Result;

use crate::Client;

pub struct Gists {
    client: Client,
}

impl Gists {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Gists { client }
    }

    /**
     * List gists for the authenticated user.
     *
     * This function performs a `GET` to the `/gists` endpoint.
     *
     * Lists the authenticated user's gists or if called anonymously, this endpoint returns all public gists:
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gists-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list(
        &self,
        since: chrono::DateTime<chrono::Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::BaseGist>> {
        let url = format!(
            "/gists?page={}&per_page={}&since={}",
            format!("{}", page),
            format!("{}", per_page),
            since,
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * Create a gist.
     *
     * This function performs a `POST` to the `/gists` endpoint.
     *
     * Allows you to add a new gist with one or more files.
     *
     * **Note:** Don't name your files "gistfile" with a numerical suffix. This is the format of the automatic naming scheme that Gist uses internally.
     *
     * FROM: <https://docs.github.com/rest/reference/gists#create-a-gist>
     */
    pub async fn create(
        &self,
        body: &crate::types::GistsCreateRequest,
    ) -> Result<crate::types::GistSimple> {
        let url = "/gists".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * List public gists.
     *
     * This function performs a `GET` to the `/gists/public` endpoint.
     *
     * List public gists sorted by most recently updated to least recently updated.
     *
     * Note: With [pagination](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination), you can fetch up to 3000 gists. For example, you can fetch 100 pages with 30 gists per page or 30 pages with 100 gists per page.
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-public-gists>
     *
     * **Parameters:**
     *
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_public(
        &self,
        since: chrono::DateTime<chrono::Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::BaseGist>> {
        let url = format!(
            "/gists/public?page={}&per_page={}&since={}",
            format!("{}", page),
            format!("{}", per_page),
            since,
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * List starred gists.
     *
     * This function performs a `GET` to the `/gists/starred` endpoint.
     *
     * List the authenticated user's starred gists:
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-starred-gists>
     *
     * **Parameters:**
     *
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_starred(
        &self,
        since: chrono::DateTime<chrono::Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::BaseGist>> {
        let url = format!(
            "/gists/starred?page={}&per_page={}&since={}",
            format!("{}", page),
            format!("{}", per_page),
            since,
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * Get a gist.
     *
     * This function performs a `GET` to the `/gists/{gist_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#get-a-gist>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn get(&self, gist_id: &str) -> Result<crate::types::GistSimple> {
        let url = format!(
            "/gists/{}",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Delete a gist.
     *
     * This function performs a `DELETE` to the `/gists/{gist_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#delete-a-gist>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn delete(&self, gist_id: &str) -> Result<()> {
        let url = format!(
            "/gists/{}",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a gist.
     *
     * This function performs a `PATCH` to the `/gists/{gist_id}` endpoint.
     *
     * Allows you to update or delete a gist file and rename gist files. Files from the previous version of the gist that aren't explicitly changed during an edit are unchanged.
     *
     * FROM: <https://docs.github.com/rest/reference/gists/#update-a-gist>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn update(
        &self,
        gist_id: &str,
        body: &crate::types::GistsUpdateRequest,
    ) -> Result<crate::types::GistSimple> {
        let url = format!(
            "/gists/{}",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * List gist comments.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/comments` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gist-comments>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_comments(
        &self,
        gist_id: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::GistComment>> {
        let url = format!(
            "/gists/{}/comments?page={}&per_page={}",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * Create a gist comment.
     *
     * This function performs a `POST` to the `/gists/{gist_id}/comments` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#create-a-gist-comment>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn create_comment(
        &self,
        gist_id: &str,
        body: &crate::types::GistsCreateCommentRequest,
    ) -> Result<crate::types::GistComment> {
        let url = format!(
            "/gists/{}/comments",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Get a gist comment.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/comments/{comment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#get-a-gist-comment>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn get_comment(
        &self,
        gist_id: &str,
        comment_id: i64,
    ) -> Result<crate::types::GistComment> {
        let url = format!(
            "/gists/{}/comments/{}",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Delete a gist comment.
     *
     * This function performs a `DELETE` to the `/gists/{gist_id}/comments/{comment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#delete-a-gist-comment>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn delete_comment(&self, gist_id: &str, comment_id: i64) -> Result<()> {
        let url = format!(
            "/gists/{}/comments/{}",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a gist comment.
     *
     * This function performs a `PATCH` to the `/gists/{gist_id}/comments/{comment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#update-a-gist-comment>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn update_comment(
        &self,
        gist_id: &str,
        comment_id: i64,
        body: &crate::types::GistsCreateCommentRequest,
    ) -> Result<crate::types::GistComment> {
        let url = format!(
            "/gists/{}/comments/{}",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * List gist commits.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/commits` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gist-commits>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_commits(
        &self,
        gist_id: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::GistCommit>> {
        let url = format!(
            "/gists/{}/commits?page={}&per_page={}",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * List gist forks.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/forks` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gist-forks>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_forks(
        &self,
        gist_id: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::GistSimple>> {
        let url = format!(
            "/gists/{}/forks?page={}&per_page={}",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * Fork a gist.
     *
     * This function performs a `POST` to the `/gists/{gist_id}/forks` endpoint.
     *
     * **Note**: This was previously `/gists/:gist_id/fork`.
     *
     * FROM: <https://docs.github.com/rest/reference/gists#fork-a-gist>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn fork(&self, gist_id: &str) -> Result<crate::types::BaseGist> {
        let url = format!(
            "/gists/{}/forks",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
     * Check if a gist is starred.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/star` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#check-if-a-gist-is-starred>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn check_is_starred(&self, gist_id: &str) -> Result<()> {
        let url = format!(
            "/gists/{}/star",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Star a gist.
     *
     * This function performs a `PUT` to the `/gists/{gist_id}/star` endpoint.
     *
     * Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
     *
     * FROM: <https://docs.github.com/rest/reference/gists#star-a-gist>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn star(&self, gist_id: &str) -> Result<()> {
        let url = format!(
            "/gists/{}/star",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
        );

        self.client.put(&url, None).await
    }

    /**
     * Unstar a gist.
     *
     * This function performs a `DELETE` to the `/gists/{gist_id}/star` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#unstar-a-gist>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn unstar(&self, gist_id: &str) -> Result<()> {
        let url = format!(
            "/gists/{}/star",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Get a gist revision.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/{sha}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#get-a-gist-revision>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `sha: &str`
     */
    pub async fn get_revision(&self, gist_id: &str, sha: &str) -> Result<crate::types::GistSimple> {
        let url = format!(
            "/gists/{}/{}",
            crate::progenitor_support::encode_path(&gist_id.to_string()),
            crate::progenitor_support::encode_path(&sha.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * List gists for a user.
     *
     * This function performs a `GET` to the `/users/{username}/gists` endpoint.
     *
     * Lists public gists for the specified user:
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gists-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_user(
        &self,
        username: &str,
        since: chrono::DateTime<chrono::Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::BaseGist>> {
        let url = format!(
            "/users/{}/gists?page={}&per_page={}&since={}",
            crate::progenitor_support::encode_path(&username.to_string()),
            format!("{}", page),
            format!("{}", per_page),
            since,
        );

        self.client.get_all_pages(&url).await
    }
}