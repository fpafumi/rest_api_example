pub mod user_module {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserVec {
        pub data: Vec<User>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User {
        pub name: String,
        pub surname: String,
        pub age: String,
        pub address: String,
    }

    impl UserVec {
        pub fn new(user_vec: Vec<User>) -> UserVec {
            UserVec {
                data: user_vec,
            }
        }
    }

    impl User {
        pub fn new(name:String, surname:String, age:String, address:String) -> User {
            User {
                name: name.to_owned(),
                surname: surname.to_owned(),
                age: age.to_owned(),
                address: address.to_owned(),
            }
        }
    }
}
