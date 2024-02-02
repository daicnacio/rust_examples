use toml::{Table,Value};
use std::fs;
use directories::BaseDirs;
pub fn set_timeout(t_out:i64)->Result<(),i8>{
    let base_dirs = BaseDirs::new().unwrap();
    let config_path = base_dirs.config_dir();
    let config_toml = config_path.join("mytoml.toml");
    println!("{:?}",&config_toml);

    let file_content = match fs::read_to_string(&config_toml) {
        Ok(r) => r,
        Err(e)=>panic!("{:?}",e),
    };
    
    let mut doc:Table= match file_content.parse::<Table>() {
        Ok(r) =>r,
        Err(e)=>panic!("{:?}",e),
    };

    doc["general"]["timeout"] = Value::Integer(t_out);
    fs::write(config_toml,doc.to_string()).expect("error writing file");
    Ok(())

}
pub fn get_timeout()->Result<i64,i8>{
    let base_dirs = BaseDirs::new().unwrap();
    let config_path = base_dirs.config_dir();
    let config_toml = config_path.join("mytoml.toml");
    let file_content = match fs::read_to_string(config_toml) {
        Ok(r) => r,
        Err(_)=>panic!("Error reading file"),
    };
    
    let doc:Table= match file_content.parse::<Table>() {
        Ok(r) =>r,
        Err(e)=>panic!("{:?}",e),
    };
    let timeout_value:i64 = doc["general"]["timeout"].as_integer().expect("error reading timeout");
    Ok(timeout_value)
}
pub fn display_user_info()->Result<String,i8>{
    Ok(String::from("OK"))
}

