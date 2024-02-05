use serde::{Serialize, Deserialize}; 
use std::ops::{Add, Sub, Mul};
#[derive(Debug, Serialize, Deserialize)]
pub struct MacrosTotal {
    pub protein: f64, 
    pub carbohydrate: f64, 
    pub fat: f64, 
    pub calories: f64
}

impl MacrosTotal {
    pub fn new(protein: f64, carbohydrate: f64, fat: f64, calories: f64) -> Self {
        Self { protein, carbohydrate, fat, calories }
    }
}

impl Add for MacrosTotal {
    type Output = Self; 
    fn add(self, other: Self) -> Self {
        Self { 
            protein: self.protein + other.protein, 
            carbohydrate: self.carbohydrate + other.carbohydrate, 
            fat: self.fat + other.fat, 
            calories: self.calories + other.calories
        }
    }
}

impl Sub for MacrosTotal {
    type Output = Self; 
    fn sub(self, other: Self) -> Self {
        Self { 
            protein: self.protein - other.protein, 
            carbohydrate: self.carbohydrate - other.carbohydrate, 
            fat: self.fat - other.fat, 
            calories: self.calories - other.calories
        }
    }
}

impl Mul<f64> for MacrosTotal {
    type Output = Self; 
    fn mul(self, scalar: f64) -> Self {
        Self { 
            protein: self.protein * scalar, 
            carbohydrate: self.carbohydrate * scalar, 
            fat: self.fat * scalar, 
            calories: self.calories * scalar
        }
    }
}