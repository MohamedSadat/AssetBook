#[allow(dead_code,unused_variables,unused_imports,unused_parens,non_snake_case)]
pub struct Asset{
    pub Id:i32,
    pub Name:String,
    pub Saved:bool
}

pub fn PrintOption(){
    println!("1- To Enter new Asset");
    println!("2- Read All Asset");
    println!("3- Find Asset");
    println!("4- Save changes");

}

fn ProcessSelection(x:i32)->i32{
    match x {
      //  1=> RegisterNewAsset(), 
        2=>println!("Reading"),
        3=>println!("Find"),
        _=>println!("else")
    }
    return x;
}