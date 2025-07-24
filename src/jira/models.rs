use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub fields: IssueFields,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueFields {
    pub summary: String,
    pub description: Option<String>,
    pub status: Status,
    pub assignee: Option<User>,
    pub reporter: Option<User>,
    pub priority: Option<Priority>,
    pub issuetype: IssueType,
    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub comment: Option<Comments>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    pub id: String,
    pub name: String,
    #[serde(rename = "statusCategory")]
    pub status_category: StatusCategory,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusCategory {
    pub id: u32,
    pub name: String,
    pub key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Priority {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueType {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comments {
    pub comments: Vec<Comment>,
    pub total: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
    pub id: String,
    pub body: String,
    pub author: User,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sprint {
    pub id: u32,
    pub name: String,
    pub state: String,
    #[serde(rename = "startDate")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(rename = "endDate")]
    pub end_date: Option<DateTime<Utc>>,
    #[serde(rename = "createdDate")]
    pub created_date: Option<DateTime<Utc>>,
    #[serde(rename = "completeDate")]
    pub complete_date: Option<DateTime<Utc>>,
    #[serde(rename = "originBoardId")]
    pub origin_board_id: Option<u32>,
    pub goal: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(rename = "projectTypeKey")]
    pub project_type_key: String,
    pub description: Option<String>,
    pub lead: Option<User>,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: Option<serde_json::Value>,
    #[serde(rename = "projectCategory")]
    pub project_category: Option<ProjectCategory>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Board {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub board_type: String,
    #[serde(rename = "self")]
    pub self_url: Option<String>,
    pub location: Option<BoardLocation>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoardLocation {
    #[serde(rename = "projectId")]
    pub project_id: Option<u32>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "projectName")]
    pub project_name: Option<String>,
    #[serde(rename = "projectKey")]
    pub project_key: Option<String>,
    #[serde(rename = "projectTypeKey")]
    pub project_type_key: Option<String>,
    #[serde(rename = "avatarURI")]
    pub avatar_uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResponse {
    pub issues: Vec<Issue>,
    pub total: u32,
    #[serde(rename = "startAt")]
    pub start_at: u32,
    #[serde(rename = "maxResults")]
    pub max_results: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transition {
    pub id: String,
    pub name: String,
    pub to: Status,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransitionsResponse {
    pub transitions: Vec<Transition>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IssueUpdate {
    pub fields: Option<serde_json::Value>,
    pub transition: Option<TransitionRequest>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransitionRequest {
    pub id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommentRequest {
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct SprintUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startDate")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endDate")]
    pub end_date: Option<DateTime<Utc>>,
}

// Agile/Software specific response models
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BoardsResponse {
    #[serde(rename = "maxResults")]
    pub max_results: u32,
    #[serde(rename = "startAt")]
    pub start_at: u32,
    pub total: u32,
    #[serde(rename = "isLast")]
    pub is_last: Option<bool>,
    pub values: Vec<Board>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SprintsResponse {
    #[serde(rename = "maxResults")]
    pub max_results: u32,
    #[serde(rename = "startAt")]
    pub start_at: u32,
    pub total: u32,
    #[serde(rename = "isLast")]
    pub is_last: Option<bool>,
    pub values: Vec<Sprint>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssuesResponse {
    #[serde(rename = "maxResults")]
    pub max_results: u32,
    #[serde(rename = "startAt")]
    pub start_at: u32,
    pub total: u32,
    #[serde(rename = "isLast")]
    pub is_last: Option<bool>,
    pub issues: Vec<Issue>,
}

// Epic model for Jira Software
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Epic {
    pub id: u32,
    pub key: String,
    pub name: String,
    pub summary: String,
    pub color: EpicColor,
    pub done: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpicColor {
    pub key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectsResponse {
    #[serde(rename = "maxResults")]
    pub max_results: u32,
    #[serde(rename = "startAt")]
    pub start_at: u32,
    pub total: u32,
    #[serde(rename = "isLast")]
    pub is_last: Option<bool>,
    pub values: Vec<Project>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpicsResponse {
    #[serde(rename = "maxResults")]
    pub max_results: u32,
    #[serde(rename = "startAt")]
    pub start_at: u32,
    pub total: u32,
    #[serde(rename = "isLast")]
    pub is_last: Option<bool>,
    pub values: Vec<Epic>,
}
