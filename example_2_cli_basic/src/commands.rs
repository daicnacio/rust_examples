use clap::Parser;


#[derive(Parser)]
#[command(author,version,about,long_about = None)]
pub struct Args{
    // with this, we need flags, without it, it is positional
    //by default uses the first letter for the short version
    #[arg(short,long)]
    pub name: String,
}