use super::doc_manager;
pub fn get_file_name(filename : &str) -> String{
    let index = filename.rfind("/").unwrap_or(0);

    filename[index..].to_string()
}
// Функции были взяты из питоновского примера, их надо проверить
pub fn get_file_name_no_ext(filename : &str) -> String{
    let filename = get_file_name(filename);
    let index = filename.rfind(".").unwrap_or(0);
    filename[..index].to_string()
}

pub fn get_file_ext(filename : &str) -> String{
    let filename = get_file_name(filename);
    let index = filename.rfind(".").unwrap_or(0);
    filename[index..].to_owned()
}

pub fn get_file_type(filename : &str){
    let ext = get_file_ext(filename);
}