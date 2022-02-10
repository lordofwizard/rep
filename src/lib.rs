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