#[allow(dead_code,unused_variables,unused_imports,unused_parens,non_snake_case)]
use std::io;
use std::{fs, io::stdin};

mod AssetModule;
fn main() {
    let mut lstasset :Vec<AssetModule::Asset>=Vec::new(); 
    let mut useropption = 10;
    let mut sel=String::new();
    println!("Welcom to asset manager!");


while useropption!=100
{
    AssetModule:: PrintOption();
    //read user option
    let useropption = ReadSelection();
    if(useropption==1)
    {
        lstasset.push(RegisterNewAsset());
        println!("Asset saved");
       // AssetModule:: PrintOption();
    }
    else if useropption==2 {
        println!("Reading");
        for i in &lstasset
        {
            println!("{}",i.Name);
        }
    }
    else if useropption==3 {
        println!("Find");
    }
    else{
        break;
    }
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
fn ReadSelection()->i32
{
    let mut sel=String::new() ;
    stdin().read_line(&mut sel).expect("didn't resceive");
    //parse it
    let useropption = sel.trim().parse::<i32>().unwrap();
    return useropption;
}
fn ParseSelection()
{

}