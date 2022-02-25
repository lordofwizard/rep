pub fn search<'a>(query:String,data: &'a String)-> String{
    let mut result : Vec<&str> = Vec::new();
    for line in data.lines(){
        if line.contains(&query.as_str()){
            result.push(line);
        }
    }
    let output : String = vec_str(&result);
    output
}
pub fn vec_str(vector : &Vec<&str>) -> String{
    let mut output :String =  String::new();
    for line in vector.iter(){
        output.push_str(line);
        output.push('\n');
    }
    output
}
pub fn search_case_sensitive<'a>(query:String,data: &'a String)-> String{
    let mut result : Vec<&str> = Vec::new();
    for line in data.lines(){
        if line.to_lowercase().contains(&query.to_lowercase().as_str()){
            result.push(line);
        }
    }
    let output : String = vec_str(&result);
    output
}
// TODO do num sensitive searching;
pub fn search_num_sensitive<'a>(query:String,data: &'a String)-> String{
    let mut result : Vec<&str> = Vec::new();
    for line in data.lines(){
        if line.to_lowercase().contains(&query.to_lowercase().as_str()){
            result.push(line);
        }
    }
    let output : String = vec_str_num(&result);
    output
}
pub fn vec_str_num(vector : &Vec<&str>) -> String{
    let mut output :String =  String::new();
    for num in 0..vector.len(){
        output.push_str(&num.to_string().as_str());
        output.push_str(" > ");
        output.push_str(vector.get(num).expect("not a line"));
        output.push('\n');
    }
    output
}
