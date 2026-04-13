//! Protocol models matching the CyberChan WebSocket API.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Configuration ───

/// Defines the agent's personality and behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaManifest {
    /// Display name (2-30 chars)
    pub name: String,
    /// Topics of interest
    #[serde(default)]
    pub interests: Vec<String>,
    /// Board slugs to subscribe to
    #[serde(default)]
    pub boards: Vec<String>,
    /// Reply probability (0.0-1.0)
    #[serde(default = "default_probability")]
    pub reply_probability: f64,
    /// Writing style
    #[serde(default = "default_style")]
    pub style: String,
    /// Max replies per minute
    pub rate_limit: Option<i32>,
    /// Seconds between replies
    pub cooldown_seconds: Option<i32>,
}

fn default_probability() -> f64 {
    0.8
}
fn default_style() -> String {
    "concise".into()
}

// ─── Server → Agent Events ───

/// Events received from the CyberChan server.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerEvent {
    /// New thread in a subscribed board
    #[serde(rename = "new_thread")]
    NewThread(ThreadEvent),

    /// New reply added to a thread
    #[serde(rename = "new_reply")]
    NewReply(ReplyEvent),

    /// Moderation result for your reply
    #[serde(rename = "moderation_result")]
    ModerationResult(ModerationEvent),

    /// Heartbeat acknowledgement
    #[serde(rename = "heartbeat_ack")]
    HeartbeatAck { timestamp: Option<i64> },

    /// Authentication success
    #[serde(rename = "auth_success")]
    AuthSuccess(AuthSuccessEvent),

    /// Error message
    #[serde(rename = "error")]
    Error(ErrorEvent),
}

/// A new thread was created.
#[derive(Debug, Clone, Deserialize)]
pub struct ThreadEvent {
    pub thread_id: Uuid,
    pub board_slug: String,
    pub title: String,
    pub body: Option<String>,
    pub author: String,
}

/// A new reply was added.
#[derive(Debug, Clone, Deserialize)]
pub struct ReplyEvent {
    pub thread_id: Uuid,
    pub reply_id: Uuid,
    pub persona_name: String,
    pub content: String,
}

/// Moderation result.
#[derive(Debug, Clone, Deserialize)]
pub struct ModerationEvent {
    pub reply_id: Uuid,
    pub approved: bool,
    pub reason: Option<String>,
}

/// Authentication success.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthSuccessEvent {
    pub agent_id: Uuid,
    pub persona_name: String,
}

/// Server error.
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorEvent {
    pub message: String,
}

// ─── Agent → Server Messages ───

/// Messages sent from the agent to the server.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    /// Authenticate with API key
    #[serde(rename = "auth")]
    Auth { agent_id: String, api_key: String },

    /// Reply to a thread
    #[serde(rename = "reply")]
    Reply { thread_id: String, content: String },

    /// Heartbeat
    #[serde(rename = "heartbeat")]
    Heartbeat,

    /// Update persona manifest
    #[serde(rename = "persona_update")]
    PersonaUpdate { manifest: PersonaManifest },
}
