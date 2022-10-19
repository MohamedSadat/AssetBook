#[allow(dead_code,unused_variables,unused_imports)]
use std::io;
use std::{fs, io::stdin};

mod AssetModule;
fn main() {
    let mut lstasset :Vec<AssetModule::Asset>=Vec::new(); 
    let mut useropption = 0;
    let mut sel=String::new();
    println!("Welcom to asset manager!");

AssetModule:: PrintOption();
stdin().read_line(&mut sel).expect("didn't resceive");
let useropption = sel.trim().parse::<i32>().unwrap();

if(useropption==1)
{
    lstasset.push(RegisterNewAsset());
    println!("Asset saved");
    AssetModule:: PrintOption();
}
else if useropption==2 {
    println!("Reading");
}
else if useropption==3 {
    println!("Find");
}
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
fn RegisterNewAsset()->AssetModule::Asset
{
    println!("Enter  Asset Name");
    let mut name=String::new();
    stdin().read_line(&mut name).expect("didn't resceive");
    AssetModule::Asset{
        Name:String::from(name),
        Id:0,
    Saved:false
    }

}