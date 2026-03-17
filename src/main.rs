use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "입력한 단어에 맞게 토시(조사)를 변환해주는 프로그램")]
struct Cli {
    /// 토시를 적용하고 싶은 단어
    #[arg(short, long, requires = "tossi", conflicts_with = "sentence")]
    word: Option<String>,

    /// 입력한 단어에 적용할 토시
    #[arg(short, long, requires = "word", conflicts_with = "sentence")]
    tossi: Option<String>,

    /// 변환된 토시만 출력
    #[arg(short, long, conflicts_with = "sentence")]
    only_tossi: bool,

    /// 토시가 포함된 문장을 변환 (예: "{철수, 은} {밥, 를} 먹습니다.")
    #[arg(short, long, conflicts_with_all = ["word", "tossi", "only_tossi"])]
    sentence: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(sentence) = &cli.sentence {
        match tossicat::modify_sentence(sentence) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("오류: {}", e),
        }
    } else if let (Some(word), Some(tossi)) = (&cli.word, &cli.tossi) {
        if cli.only_tossi {
            match tossicat::transform(word, tossi) {
                Ok((_word, tossi)) => println!("{}", tossi),
                Err(e) => eprintln!("오류: {}", e),
            }
        } else {
            match tossicat::postfix(word, tossi) {
                Ok(result) => println!("{}", result),
                Err(e) => eprintln!("오류: {}", e),
            }
        }
    } else {
        eprintln!("-w/-t 또는 -s 옵션을 지정해주세요. --help로 사용법을 확인하세요.");
    }
}
