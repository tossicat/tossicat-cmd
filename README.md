# Tossi for Rust

이 프로젝트는 임의의 단어와 그 단어에 붙일 조사를 입력하면, 입력한 조사를 같이 입력한 단어에 자연스러운 형태로 바꿔 반환해 주는 CLI(Command-Line Interface, 이하 커맨드 라인 인터페이스)에서 작동하는 앱을 만드는 프로젝트입니다. 코드는 Rust로 작성하고 있으며, [TossiCat core](https://github.com/tossicat/tossicat-core)을 라이브러리로 이용해 대부분의 중요 기능을 가져다 사용하고 있습니다. 자세한 내용은 아래를 내용을 참고하세요.

## 컴파일 하기

### `cargo run`로 컴파일한 다음 사용하는 법

```rust
cargo run -- --word 테스트  --tossi 은
cargo run -- --word 테스트  --tossi 은 -o
```

구체적인 사용법 보기

```rust
➜ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/tossi --help`
tossi 0.1.0
Tossi(토시)는 사용자가 입력한 단어와 토시를 입력하였을 때, 입력한 토시를 입력한 단어에 적절하게 변환해주는 프로그램입니다

USAGE:
    tossi [OPTIONS] --word <단어> --tossi <토시>

OPTIONS:
    -h, --help                       Print help information
    -o, --only-tossi <ONLY_TOSSI>    반환 값에 사용자가 입력한 단어 적용 유무 [possible values: true, false]
    -t, --tossi <토시>               입력한 단어에 적용할 토시
    -V, --version                    Print version information
    -w, --word <단어>                토시를 적용하고 싶은 단어
```

### `cargo build`로 컴파일한 다음 사용하는 법

```rust
target/debug/tossi -h
target/debug/tossi -t 을 -w 나뭇가지
target/debug/tossi -t 을 -w 나뭇가지 -o
```

구체적인 사용법 보기

```rust
➜ target/debug/tossi -h
tossi 0.1.0
Tossi(토시)는 사용자가 입력한 단어와 토시를 입력하렸을 때, 입력한 단어에 적합한

USAGE:
    tossi [OPTIONS] --word <단어> --tossi <토시>

OPTIONS:
    -h, --help                       Print help information
    -o, --only-tossi <ONLY_TOSSI>    반환 값에 사용자가 입력한 단어 적용 유무 [possible values: true, false]
    -t, --tossi <토시>               입력한 단어에 적용할 토시
    -V, --version                    Print version information
    -w, --word <단어>                토시를 적용하고 싶은 단어
```

### 이 프로젝트를 빌드하기

이 프로젝트를 빌드하기 위해서는 다음 명령어를 실행하면 됩니다.

```console
cargo build --release
```

위의 명령어를 통해 빌드했으면 `tossicat-cmd/target/release` 폴더에 `tossi`이라는 이름으로 실행 파일이 만들어졌을 것이다. 의 폴더로 이동한 다음 다음과 같이 실행하시면 됩니다. 참고로 이 파일을 다른 폴더에 이동해도 적절하게 작동합니다.

```console
➜ ./tossi -h
tossi 0.1.0
Tossi(토시)는 사용자가 입력한 단어와 토시를 입력하였을 때, 입력한 토시를 입력한 단어에 적절하게 변환해주는 프로그램입니다

USAGE:
    tossi [OPTIONS] --word <단어> --tossi <토시>

OPTIONS:
    -h, --help            Print help information
    -o, --only-tossi      반환 값에 사용자가 입력한 단어 적용 유무
    -t, --tossi <토시>    입력한 단어에 적용할 토시
    -V, --version         Print version information
    -w, --word <단어>     토시를 적용하고 싶은 단어
```

## 코드 작성에서 유의할 점

코딩 스타일을 맞추기 위해서 코드를 올리기 전에 다음 명령어를 이용하여 코드를 정리하여 올립니다.

```console
cargo fmt
```
