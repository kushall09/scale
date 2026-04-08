use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
     #[arg(short, long)]
    scale:i32,
     #[arg(short, long)]
    measure:f32,
     #[arg(short, long, default_value_t = 1)]
    count:u8,
}
fn main() 
{
let _args= Args::parse();
       // println!("The value of {} is yeti",args.scale);
let value;
    if _args.scale==50{
        value=_args.measure/50.0;
       println!("\x1b[1m{}mm\x1b[0m",value); 
    }
    else if _args.scale==100{
        value=_args.measure/100.0;
       println!("\x1b[1m{}mm\x1b[0m",value); 
    }
    else if _args.scale==200{
        value=_args.measure/200.0;
       println!("\x1b[1m{}mm\x1b[0m",value); 
    }
    else if _args.scale==300{
        value=_args.measure/300.0;
       println!("\x1b[1m{}mm\x1b[0m",value); 
    }
}


