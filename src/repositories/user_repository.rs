use crate::entities::user_entity;
use csv::Reader;
use user_entity::{User, UserVec};

pub fn get_users(filename: &str) -> Result<UserVec, csv::Error> {
    let mut reader = Reader::from_path(filename).unwrap();
    let mut users = Vec::new();
    for record in reader.records() {
        for row in &record.unwrap() {
            let mut col = Vec::new();
            for i in row.split(";") {
                col.push(i);
            }
            users.push(User::new(
                col.get(0).unwrap().parse().unwrap(),
                col.get(1).unwrap().parse().unwrap(),
                col.get(2).unwrap().parse().unwrap(),
                col.get(3).unwrap().parse().unwrap(),
            ));
        }
    }
    return Ok(UserVec::new(users));
}
