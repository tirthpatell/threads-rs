use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Earliest Unix timestamp usable for insights queries.
pub const MIN_INSIGHT_TIMESTAMP: i64 = 1_712_991_600;

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Available post insight metrics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostInsightMetric {
    #[serde(rename = "views")]
    Views,
    #[serde(rename = "likes")]
    Likes,
    #[serde(rename = "replies")]
    Replies,
    #[serde(rename = "reposts")]
    Reposts,
    #[serde(rename = "quotes")]
    Quotes,
    #[serde(rename = "shares")]
    Shares,
}

/// Available account insight metrics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountInsightMetric {
    #[serde(rename = "views")]
    Views,
    #[serde(rename = "likes")]
    Likes,
    #[serde(rename = "replies")]
    Replies,
    #[serde(rename = "reposts")]
    Reposts,
    #[serde(rename = "quotes")]
    Quotes,
    #[serde(rename = "clicks")]
    Clicks,
    #[serde(rename = "followers_count")]
    FollowersCount,
    #[serde(rename = "follower_demographics")]
    FollowerDemographics,
}

/// Time period for insights.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsightPeriod {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "lifetime")]
    Lifetime,
}

/// Breakdown options for follower demographics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FollowerDemographicsBreakdown {
    #[serde(rename = "country")]
    Country,
    #[serde(rename = "city")]
    City,
    #[serde(rename = "age")]
    Age,
    #[serde(rename = "gender")]
    Gender,
}

// ---------------------------------------------------------------------------
// Option structs
// ---------------------------------------------------------------------------

/// Options for post insights requests.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PostInsightsOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<PostInsightMetric>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<InsightPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
}

/// Options for account insights requests.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccountInsightsOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<AccountInsightMetric>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<InsightPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
    /// For `follower_demographics`: country, city, age, or gender.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<FollowerDemographicsBreakdown>,
}

// ---------------------------------------------------------------------------
// Response structs
// ---------------------------------------------------------------------------

/// An individual analytics metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub name: String,
    pub period: String,
    #[serde(default)]
    pub values: Vec<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_value: Option<TotalValue>,
}

/// A metric value with optional timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub value: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

/// An aggregated metric value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalValue {
    pub value: i64,
}
