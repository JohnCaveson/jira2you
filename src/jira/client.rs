use reqwest::{Client, Method};
use serde_json::json;
use crate::jira::models::*;
use anyhow::Result;

pub struct JiraClient {
    client: Client,
    username: String,
    api_token: String,
    domain: String,
}

impl JiraClient {
    pub fn new(username: String, api_token: String, domain: String) -> Self {
        let client = Client::new();
        Self { client, username, api_token, domain }
    }

    pub async fn get_issue(&self, issue_id: &str) -> Result<Issue> {
        self.send_request(Method::GET, &format!("/issue/{}", issue_id), None).await
    }

    pub async fn get_sprint_issues(&self, board_id: u32, sprint_id: u32) -> Result<Vec<Issue>> {
        let response: IssuesResponse = self
            .send_agile_request(
                Method::GET,
                &format!(
                    "/board/{}/sprint/{}/issue",
                    board_id, sprint_id
                ),
                None,
            )
            .await?;

        Ok(response.issues)
    }

    pub async fn get_backlog(&self, board_id: u32) -> Result<Vec<Issue>> {
        let response: SearchResponse = self
            .send_agile_request(
                Method::GET,
                &format!("/board/{}/backlog", board_id),
                None,
            )
            .await?;

        Ok(response.issues)
    }

    pub async fn get_transitions(&self, issue_id: &str) -> Result<Vec<Transition>> {
        let response: TransitionsResponse = self
            .send_request(
                Method::GET,
                &format!("/issue/{}/transitions", issue_id),
                None,
            )
            .await?;

        Ok(response.transitions)
    }

    pub async fn transition_issue(&self, issue_id: &str, transition_id: &str) -> Result<()> {
        let update = IssueUpdate {
            fields: None,
            transition: Some(TransitionRequest {
                id: transition_id.to_string(),
            }),
        };

        self.send_request(
            Method::POST,
            &format!("/issue/{}/transitions", issue_id),
            Some(json!(update)),
        )
        .await
        .map(|_: serde_json::Value| ())
    }

    pub async fn update_issue(&self, issue_id: &str, update: IssueUpdate) -> Result<()> {
        self.send_request(
            Method::PUT,
            &format!("/issue/{}", issue_id),
            Some(json!(update)),
        )
        .await
        .map(|_: serde_json::Value| ())
    }

    pub async fn add_comment(&self, issue_id: &str, comment: &str) -> Result<()> {
        self.send_request(
            Method::POST,
            &format!("/issue/{}/comment", issue_id),
            Some(json!(CommentRequest {
                body: comment.to_string(),
            })),
        )
        .await
        .map(|_: serde_json::Value| ())
    }

    // New Jira Software specific methods
    pub async fn get_projects(&self) -> Result<Vec<Project>> {
        let response: ProjectsResponse = self
            .send_request(Method::GET, "/project", None)
            .await?;
        Ok(response.values)
    }

    pub async fn get_boards(&self) -> Result<Vec<Board>> {
        let response: BoardsResponse = self
            .send_agile_request(Method::GET, "/board", None)
            .await?;
        Ok(response.values)
    }

    pub async fn get_board(&self, board_id: u32) -> Result<Board> {
        self.send_agile_request(Method::GET, &format!("/board/{}", board_id), None)
            .await
    }

    pub async fn get_board_sprints(&self, board_id: u32) -> Result<Vec<Sprint>> {
        let mut all_sprints = Vec::new();
        let mut start_at = 0;
        loop {
            let response: SprintsResponse = self
                .send_agile_request(
                    Method::GET,
                    &format!("/board/{}/sprint?startAt={}", board_id, start_at),
                    None,
                )
                .await?;

            all_sprints.extend(response.values);

            if response.is_last.unwrap_or(true) {
                break;
            }
            start_at = response.start_at + response.max_results;
        }
        Ok(all_sprints)
    }

    pub async fn get_sprint(&self, sprint_id: u32) -> Result<Sprint> {
        self.send_agile_request(Method::GET, &format!("/sprint/{}", sprint_id), None)
            .await
    }

    pub async fn update_sprint(&self, sprint_id: u32, update: &SprintUpdate) -> Result<Sprint> {
        self.send_agile_request(
            Method::POST,
            &format!("/sprint/{}", sprint_id),
            Some(json!(update)),
        )
        .await
    }

    pub async fn get_board_epics(&self, board_id: u32) -> Result<Vec<Epic>> {
        let response: EpicsResponse = self
            .send_agile_request(
                Method::GET,
                &format!("/board/{}/epic", board_id),
                None,
            )
            .await?;
        Ok(response.values)
    }

    pub async fn get_epic_issues(&self, epic_id: u32) -> Result<Vec<Issue>> {
        let response: IssuesResponse = self
            .send_agile_request(
                Method::GET,
                &format!("/epic/{}/issue", epic_id),
                None,
            )
            .await?;
        Ok(response.issues)
    }

    // Private Methods
    async fn send_request<T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        let api_base = format!("{}/rest/api/3", self.domain.trim_end_matches('/'));
        let url = format!("{}{}", api_base, path);
        let request = self
            .client
            .request(method, &url)
            .basic_auth(&self.username, Some(&self.api_token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");

        let request = if let Some(body) = body {
            request.json(&body)
        } else {
            request
        };

        Ok(request.send().await?.error_for_status()?.json().await?)
    }

    async fn send_agile_request<T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        let api_base = format!("{}/rest/agile/1.0", self.domain.trim_end_matches('/'));
        let url = format!("{}{}", api_base, path);
        let request = self
            .client
            .request(method, &url)
            .basic_auth(&self.username, Some(&self.api_token))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");

        let request = if let Some(body) = body {
            request.json(&body)
        } else {
            request
        };

        Ok(request.send().await?.error_for_status()?.json().await?)
    }
}

