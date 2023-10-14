use crate::entities::city_entity::CityVec;
use crate::repositories::city_repository;

pub fn get_cities_json() -> String {
    let city_csv: String =
        std::env::var("CITY_CSV").expect("the CITY_CSV is not present in .env file");
    let city_csv = &city_csv as &str;
    let cities: CityVec = city_repository::get_cities(city_csv).unwrap();
    serde_json::to_string(&cities).unwrap()
}
