mod iniconfig;
fn main() {
    println!("User info:");
    println!("{}",iniconfig::display_user_info().unwrap());
    iniconfig::set_timeout(500).unwrap();
    let t_out:i64 = iniconfig::get_timeout().unwrap();
    println!("timeout: {}",t_out);
}
