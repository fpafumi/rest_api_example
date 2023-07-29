use crate::repositories::city_repository;
use crate::entities::city_entity::CityVec;

pub fn get_cities_json() -> String {
    let cities: CityVec =
        city_repository::get_cities("/home/furetto/Scrivania/progetti/applications/rest_api/raw/city.csv")
            .unwrap();
    serde_json::to_string(&cities).unwrap()
}