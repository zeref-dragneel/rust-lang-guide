 pub fn run(){
     let name = "sharath";
     let mut age = 23; //if you dont do this you wont be able to reassign a value
     age=age+1;
     //multiple variable declaration
     let (des1,des2)=("ux designer","dev ops engineer");
     println!("My name is {} and I am {}, I work as {} & {}",name,age,des1,des2)
 }