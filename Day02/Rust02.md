# study-rust
러스트 학습 리포지토리

## Day 02

## Cargo.toml
Cargo명령어로 프로젝트 생성 후 프로젝트에 추가되는 설정파일
```shell
[package]
name = "rust02"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

### 변수와 데이터타입
- 변수 선언
	- 키워드 let 사용

	```rust
	let a = 100;
    let b = 3.141592;

    println!("int   : {}", a);
    println!("float : {}", b);
	```

- 각 타입형

	|크기 |Signed |Unsigned |
	|---|---|---|
	|8비트 |i8 |u8 |
	|16비트 |i16 |u16 |
	|32비트 |i32 |u32 |
	|64비트 |i64 |u64 |
	|128비트 |i128 |u128 |
	|아키텍처별 |isize |usize |
	|---|---|---|
	|32비트|f32| |
	|64비트|f64| |
	|---|---|---|
	|bool|true/false| |

	```rust
	// 타입 지정
    let c: i16 = 12345;
    let d: f64 = 12.45726;
    let e: bool = false;

    println!("int   : {}", c);
    println!("float : {}", d);
    println!("bool  : {}", e);
	```

- 


[Back](https://github.com/hugoMGSung/study-rust/blob/main/README.md)