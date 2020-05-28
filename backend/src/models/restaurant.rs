use std::convert::From;

#[derive(Debug, Serialize, Deserialize)]
pub struct Restaurant {
    pub name: String,
    pub ratings: Vec<i32>,
    #[serde(rename="avgRating")]
    pub avg_rating: f32
}

impl From<postgres::Row> for Restaurant {
    fn from(row: postgres::Row) -> Self {
        Restaurant {
            name: row.get(0),
            ratings: row.get(1),
            avg_rating: row.get(2)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Rating {
    pub name: String,
    pub rating: i32,
}
