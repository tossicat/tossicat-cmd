use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "입력한 단어에 맞게 토시(조사)를 변환해주는 프로그램")]
struct Cli {
    /// 토시를 적용하고 싶은 단어
    #[arg(short, long)]
    word: String,

    /// 입력한 단어에 적용할 토시
    #[arg(short, long)]
    tossi: String,

    /// 변환된 토시만 출력
    #[arg(short, long, default_value_t = false)]
    only_tossi: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.only_tossi {
        match tossicat::transform(&cli.word, &cli.tossi) {
            Ok((_word, tossi)) => println!("{}", tossi),
            Err(e) => eprintln!("오류: {}", e),
        }
    } else {
        match tossicat::postfix(&cli.word, &cli.tossi) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("오류: {}", e),
        }
    }
}
