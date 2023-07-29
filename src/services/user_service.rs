use crate::repositories::user_repository;
use crate::entities::user_entity::UserVec;

pub fn get_user_json() -> String {
    let users: UserVec =
        user_repository::get_users("/home/furetto/Scrivania/progetti/applications/rest_api/raw/user.csv")
            .unwrap();
    serde_json::to_string(&users).unwrap()
}