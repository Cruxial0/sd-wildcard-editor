static OFFSET: i32 = 2;

pub fn to_comma_seperated(input: &Vec<String>) -> String{
    if input.len() <= 0 { return "".to_owned() }
    if input.len() == 1 { return input[0].clone();}
    let val: String = input.iter().map(|x| format!("{}, ", x)).collect();
    let index = (val.len() as i32) - OFFSET;

    let split = val.split_at(index as usize);
    split.0.to_owned()
}

pub fn rusqlite_value_to_csv(input: &Vec<rusqlite::types::Value>) -> String{
    let val = input.iter().map(|x| format!("{:?}", x)).collect();
    to_comma_seperated(&val)
}