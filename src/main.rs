#[allow(dead_code,unused_variables,unused_imports,unused_parens,non_snake_case)]
use std::io;
use std::{fs, io::stdin};

mod AssetModule;
fn main() {
    let mut lstasset :Vec<AssetModule::Asset>=Vec::new(); 
    let mut useropption = 0;
    let mut sel=String::new();
    println!("Welcom to asset manager!");

AssetModule:: PrintOption();
//read user option
stdin().read_line(&mut sel).expect("didn't resceive");
//parse it
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