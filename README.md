# Tossi for Rust

이 프로젝트는 임의의 단어와 그 단어에 붙일 조사를 입력하면, 입력한 조사를 같이 입력한 단어에 자연스러운 형태로 바꿔 반환해 주는 CLI(Command-Line Interface)에서 작동하는 앱입니다. 코드는 Rust로 작성하고 있으며, [TossiCat core](https://github.com/tossicat/tossicat-core)을 라이브러리로 이용해 대부분의 중요 기능을 가져다 사용하고 있습니다.

## 사용법

### 단어에 토시 적용

```console
$ tossi -w 사과 -t 을
사과를

$ tossi -w 집 -t 으로
집으로

$ tossi -w 테스트 -t 은
테스트는
```

변환된 토시만 출력:

```console
$ tossi -w 사과 -t 을 -o
를
```

### 문장 변환

`{단어, 토시}` 형식이 포함된 문장을 한 번에 변환할 수 있습니다.

```console
$ tossi -s "{철수, 은} {영희, 과} {밥, 를} 먹습니다."
철수는 영희와 밥을 먹습니다.
```

### 도움말 보기

```console
$ tossi --help
입력한 단어에 맞게 토시(조사)를 변환해주는 프로그램

Usage: tossi [OPTIONS]

Options:
  -w, --word <WORD>          토시를 적용하고 싶은 단어
  -t, --tossi <TOSSI>        입력한 단어에 적용할 토시
  -o, --only-tossi           변환된 토시만 출력
  -s, --sentence <SENTENCE>  토시가 포함된 문장을 변환 (예: "{철수, 은} {밥, 를} 먹습니다.")
  -h, --help                 Print help
  -V, --version              Print version
```

## 빌드하기

### 개발 빌드

```console
cargo build
cargo run -- -w 테스트 -t 은
```

### 릴리스 빌드

```console
cargo build --release
```

빌드하면 `target/release` 폴더에 `tossi` 실행 파일이 생성됩니다. 이 파일을 다른 폴더에 이동해도 작동합니다.

## 코드 작성에서 유의할 점

코딩 스타일을 맞추기 위해서 코드를 올리기 전에 다음 명령어를 이용하여 코드를 정리하여 올립니다.

```console
cargo fmt
```

## 라이선스

MIT
