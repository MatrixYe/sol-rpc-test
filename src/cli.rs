/// @Name cli
///
/// @Date 2024/8/13 下午2:22
///
/// @Author Matrix.Ye
///
/// @Description:

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = 3)]
    pub count: usize,

    pub url: String,
}