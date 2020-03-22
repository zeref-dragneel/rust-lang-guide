/*
    primitive stings --> immutable
    string --> growable, heap allocated

*/
pub fn run(){
   // let name = "sharath"; //primitive
    //name = "Sharath Mohan"; //throws an error
    let mut name = String::from("sharath");
    name.push_str(" Mohan");
    println!("{}",name);
}