pub struct Asset{
    pub Id:i32,
    pub Name:String,
    pub Saved:bool
}

pub fn PrintOption(){
    println!("1- To Enter new Asset");
    println!("2- Read All Asset");
    println!("3- Find Asset");
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