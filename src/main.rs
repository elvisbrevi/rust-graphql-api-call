use gql_client::Client;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
struct Post {
    title: String,
    slug: String,
    #[serde(rename = "dateAdded")]
    date_added: String,
}

#[derive(Deserialize, Serialize)]
struct Publication {
    posts: Vec<Post>,
}

#[derive(Deserialize, Serialize)]
struct User {
    publication: Publication,
}

#[derive(Deserialize, Serialize)]
struct Data {
    user: User,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = "
    {
        user(username:\"elvisbrevi\") {
            publication {
                posts {
                    title
                    slug
                    dateAdded
                }
            }
        }
    }
  ";

    let endpoint = "https://api.hashnode.com";
    let client = Client::new(endpoint);
    let response = client.query::<Data>(query).await.unwrap();

    // for publication in &response {
    //     println!("{}", publication.user.publication.posts[0].title);
    //     println!("{}", publication.user.publication.posts[0].slug);
    //     println!("{}", publication.user.publication.posts[0].date_added);
    // }

    // Serialize the response to a pretty-printed JSON string and print it
    let formatted_response = serde_json::to_string_pretty(&response)?;
    println!("{}", formatted_response);

    Ok(())
}
