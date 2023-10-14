use crate::entities::user_entity::UserVec;
use crate::repositories::user_repository;

pub fn get_user_json() -> String {
    let user_csv: String =
        std::env::var("USER_CSV").expect("the USER_CSV is not present in .env file");
    let user_csv = &user_csv as &str;
    let users: UserVec = user_repository::get_users(user_csv).unwrap();
    serde_json::to_string(&users).unwrap()
}
