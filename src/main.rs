#[allow(dead_code,unused_variables,unused_imports,unused_parens,non_snake_case)]
use std::io;
use std::{fs, io::stdin};
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, ErrorKind};
use std::path::Path;
use std::io::{Write,BufRead};
mod AssetModule;
fn main() {
    let args: Vec<String> = env::args().collect();
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
        println!("Reading asset from database");
        for i in &lstasset
        {
       //     println!("{}",i.Name);
        }
        ReadAssets();
    }
    else if useropption==4{
        SaveChanges(&lstasset);
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
fn SaveChanges(l:&Vec<AssetModule::Asset>){
let mut sql=String::new();
let path = "asset.txt";
    for i in l
    {

    sql.push_str(format!("{} , {}  ",i.Id,i.Name).to_string().as_str());
    }

    let output=File::create(path);
    let mut output=match output {
    Ok(file)=>file,
    Err(error)=> {
        panic!("problem create file{:?}",error);
    }                
    
};
    //write!(&mut output,"hello");
    fs::write(path, sql.as_str());
}
fn ReadAssets(){
let content = fs::read_to_string("asset.txt").expect("File not fount");
println!("{:?}",content);

}

