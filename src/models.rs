
#[derive(Queryable)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
}
