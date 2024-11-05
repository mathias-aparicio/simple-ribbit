use axum::{extract::Path, Json};

use crate::schemas::NewPost;
// In the future each file should handle routes and schemas

/// Defines each pages

// Use axum_login for authentification or build your own

// Add to the the redis data base an user {username, password} to test login and logout
pub async fn home() -> &'static str {
	"Hello World"
}

// takes the data of the form as a json object
// Json is a axum extractor that extracts the body of the request as a json object
pub async fn publish_post(/*form: Json<NewPost>*/) {}

// slug is retrieved by the link that looks like /post/:slug
pub async fn get_post_by_slug(Path(slug): Path<String>) {}

pub async fn sign_in() {
	// 1 Check if the email or username is in the db
	// 2 Create a hash of the password
	// 3 Create a new user and save it to the db
	// 4 Return a html reason with a token in the header so that the user can log in

}

// 1. Check if password and username are in db
// 2. Create an access tokens and a refresh tokens and store it in the cookies might want to store it in the db
// 3. return a response https://medium.com/@Rushabh_/backend-part-2-user-authentication-78a51ea122b2
pub async fn login() {}


