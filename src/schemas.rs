// In the future each file should handle routes and schemas

pub struct AuthorInfo {
    pub name: String,
}
// Represents a post as stored in the database
pub struct Post {
    pub title: String,
    pub slug: String,
    pub author: AuthorInfo,
    pub body: String,
}

// Represents a post as written by the user
pub struct NewPost {
    pub title: String,
    pub body: String,
}
