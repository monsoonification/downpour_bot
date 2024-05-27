use std::collections::HashMap;
use calamine::{open_workbook, Reader, Xlsx};
use serenity::json::to_string;

pub struct TriggerMap {
    map: HashMap<String, String>,
} //TODO: add support for multiple keyword triggers later, word to response matching for now

impl TriggerMap {
    
    pub fn new() -> Self {
        TriggerMap {
            map: HashMap::new(),
        }
    }

    /*
     * Case-insensitive insert function, these are probably all redundant but i'm procrastinging writging harder stuff 
     */
    fn insert(&mut self, key: String, value: String) {
        let lowercase_key = key.to_lowercase();
        self.map.insert(lowercase_key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        let lowercase_key = key.to_lowercase();
        if let Some(result) = self.map.get(&lowercase_key) {
            return Some(result.clone());
        }
        return None;
    }

    /*
     * Case-insensitive get function
     */
    fn remove(&mut self, key: String) -> bool {
        let lowercase_key = key.to_lowercase();
        return self.map.remove(&lowercase_key) != None;
    }
}

//spreedsheet will be online in the future maybe? dunno how to d otaht yet 
pub fn xlsx_to_hashmap(filepath: &str) -> TriggerMap {
    let mut map: TriggerMap = TriggerMap::new();
    let mut excel: Xlsx<_> = open_workbook(filepath).unwrap();
    if let Ok(r) = excel.worksheet_range("Sheet1") {
        println!("sheet found!");
        for row in r.rows() {
            let cell_1 = row[0].to_string();
            let cell_2 = row[1].to_string();
            if cell_1 != "" || cell_2 != "" {
                map.insert(cell_1, cell_2);
                println!("1 row inserted");
            }
            else {
                println!("something went wrong...");
            }
            
        }
    }
    return map;
}




//DEPRECATED I WILL REWRITE LATER
fn contains_keywords_ignorecase(string: String, keywords: &[&str]) -> bool {
    
    for keyword in keywords {
        let lowercase_string = string.to_lowercase();
        if lowercase_string.contains(&keyword.to_lowercase()) {
            return true;
        }
    }
    return false;
}