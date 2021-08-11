use anyhow::Result;

use crate::Client;

pub struct Users {
    client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Users { client }
    }

    /**
     * Get User Info by User ID.
     *
     * This function performs a `GET` to the `/users/<id>` endpoint.
     *
     * Retrieve the information of the user with the matching user ID.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get_users_user_id(&self) -> Result<crate::types::User> {
        let url = "/users/<id>".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Suspend a user.
     *
     * This function performs a `DELETE` to the `/users/<id>` endpoint.
     *
     * Suspends a user. Does not delete the user's cards. Currently this action is not reversible.
     */
    pub async fn delete_users_id(&self) -> Result<()> {
        let url = "/users/<id>".to_string();
        self.client.delete(&url, None).await
    }

    /**
     * Modify Existing User.
     *
     * This function performs a `PATCH` to the `/users/<id>` endpoint.
     *
     * Modify information about a user.
     */
    pub async fn patch_users_id(&self, body: &crate::types::PatchUsersIdRequest) -> Result<()> {
        let url = "/users/<id>".to_string();
        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * List users.
     *
     * This function performs a `GET` to the `/users` endpoint.
     *
     * Retrieve all users of the business.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     * * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     * * `department_id: &str`
     * * `location_id: &str`
     */
    pub async fn get_users(
        &self,
        start: &str,
        page_size: f64,
        department_id: &str,
        location_id: &str,
    ) -> Result<Vec<crate::types::User>> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !department_id.is_empty() {
            query_args.push(format!("department_id={}", department_id));
        }
        if !location_id.is_empty() {
            query_args.push(format!("location_id={}", location_id));
        }
        query_args.push(format!("page_size={}", page_size));
        if !start.is_empty() {
            query_args.push(format!("start={}", start));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/users?{}", query);

        let resp: crate::types::GetUsersResponse = self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.data)
    }

    /**
     * List users.
     *
     * This function performs a `GET` to the `/users` endpoint.
     *
     * As opposed to `get_users`, this function returns all the pages of the request at once.
     *
     * Retrieve all users of the business.
     */
    pub async fn get_all_users(
        &self,
        department_id: &str,
        location_id: &str,
    ) -> Result<Vec<crate::types::User>> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !department_id.is_empty() {
            query_args.push(format!("department_id={}", department_id));
        }
        if !location_id.is_empty() {
            query_args.push(format!("location_id={}", location_id));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/users?{}", query);

        self.client.get_all_pages(&url, None).await
    }

    /**
     * Invite a new user.
     *
     * This function performs a `POST` to the `/users/deferred` endpoint.
     *
     * Creates an invite for the user to accept. Also optionally sets department, location, and manager.
     */
    pub async fn post_users_deferred(
        &self,
        body: &crate::types::PostUsersDeferredRequest,
    ) -> Result<crate::types::User> {
        let url = "/users/deferred".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Get status of a deferred user task.
     *
     * This function performs a `GET` to the `/users/deferred/status/<id>` endpoint.
     *
     * Gets status of a deferred task for users
     */
    pub async fn get_users_deferred_status_id(
        &self,
    ) -> Result<crate::types::GetUsersDeferredStatusIdResponse> {
        let url = "/users/deferred/status/<id>".to_string();
        self.client.get(&url, None).await
    }
}