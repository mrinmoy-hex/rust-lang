use auth_service::auth_utils::models::Credentials;
use auth_service::authenticate;

fn main() {
    // Use placeholder values in this learning example, never real account credentials.
    let demo_credentials = Credentials {
        username: String::from("demo-user"),
        passwd: String::from("not-a-real-password"),
    };

    // `authenticate` takes ownership of the credentials value.
    authenticate(demo_credentials);
}
