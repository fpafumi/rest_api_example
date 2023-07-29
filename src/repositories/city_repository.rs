use crate::entities::city;
use csv::Reader;
use city::{City, CityVec};

pub fn get_cities(filename: &str) -> Result<CityVec, csv::Error> {
    let mut reader = Reader::from_path(filename).unwrap();
    let mut cities = Vec::new();
    for record in reader.records() {
        for row in &record.unwrap() {
            let mut col = Vec::new();
            for i in row.split(";") {
                col.push(i);
            }
            cities.push(City::new(
                col.get(0).unwrap().parse().unwrap(),
                col.get(1).unwrap().parse().unwrap(),
                col.get(2).unwrap().parse().unwrap(),
            ));
        }
    }
    return Ok(CityVec::new(cities));
}
