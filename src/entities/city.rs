use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CityVec {
    pub data: Vec<City>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    pub name: String,
    pub nation: String,
    pub region: String,
}

impl CityVec {
    pub fn new(city_vec: Vec<City>) -> CityVec {
        CityVec { data: city_vec }
    }
}

impl City {
    pub fn new(name: String, nation: String, region: String) -> City {
        City {
            name: name.to_owned(),
            nation: nation.to_owned(),
            region: region.to_owned(),
        }
    }
}
