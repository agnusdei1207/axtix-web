#[derive(Serialize, Deserialize)]
pub struct Notice {
    pub title: String,
    pub content: String,
    pub createdAt: DateTime<Utc>,
    pub deletedAt: DateTime<Utc>,
}
