use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct LeaderboardDefinition {
    pub id: String,
    pub query: LeaderboardQuery,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LeaderboardQuery {
    Sql {
        query: String,
        player: String,
        value: String,
        value_type: ValueType,
        #[serde(default)]
        value_format: ValueFormat,
    },
    Statistic {
        namespace: String,
        key: String,
        aggregate: Aggregate,
        ranking: Ranking,
        #[serde(default)]
        value_format: ValueFormat,
    },
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Aggregate {
    Total,
    Average,
    Minimum,
    Maximum,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Ranking {
    Lowest,
    Highest,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ValueType {
    Int,
    #[serde(rename = "uint")]
    UInt,
    Float,
}

#[derive(Default, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ValueFormat {
    #[default]
    Count,
    Time,
}
