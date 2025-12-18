use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub date: NaiveDate,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {} {:?}", self.id, self.name, self.age, self.date)
    }
}

pub fn create_person_collection() -> Vec<Person> {
    vec![
        Person {
            id: 1,
            name: "Mario".to_string(),
            age: 43,
            date: NaiveDate::from_ymd_opt(1981, 2, 21).unwrap(),
        },
        Person {
            id: 2,
            name: "Luigi".to_string(),
            age: 41,
            date: NaiveDate::from_ymd_opt(1983, 3, 25).unwrap(),
        },
    ]
}
