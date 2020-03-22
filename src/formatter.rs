pub fn run(){
    //generic
    println!("this is a printed via format specifier {}","rust is awesome");
    //positinal
    println!("{0} is amazing {0}","rust");
    //Named
    println!("{lang} is amazing",lang="Rust");
    //debugger trait
    println!("{:?} is amazing",(12,true,"hello"));
}