use csv::Writer;   
use super::super::models::{food_normalized::FoodNormalized, food_normalized::FoodNormalizedCsv}; 

pub async fn write_csv_file(file_path: String, content: Vec<FoodNormalized>) -> Result<(), std::io::Error> {
    let mut writer = Writer::from_path(file_path)?; 
    for food in content {
        writer.serialize(food)?; 
    }
    writer.flush()?; 

    Ok(())
}

pub async fn read_csv_file(file_path: String) -> Result<Vec<FoodNormalizedCsv>, std::io::Error> {
    let mut reader = csv::Reader::from_path(file_path)?; 
    let mut food_normalized: Vec<FoodNormalizedCsv> = Vec::new(); 
    for result in reader.deserialize() {
        let food: FoodNormalizedCsv = result?; 
        food_normalized.push(food); 
    }
    Ok(food_normalized)
}
