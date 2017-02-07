
pub fn filprintln(filter:&str,source: &String){

    if source.contains(filter){
        println!("{}",source);
    }
}