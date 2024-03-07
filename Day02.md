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
	|**정수형** |
	|8비트 |i8 |u8 |
	|16비트 |i16 |u16 |
	|32비트 |i32 |u32 |
	|64비트 |i64 |u64 |
	|128비트 |i128 |u128 |
	|아키텍처별 |isize |usize |
	|**부동소수점** |
	|32비트|f32| |
	|64비트|f64| |
	|**불타입** |
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

- 변수인데 바꿀 수 없다!!
	```rust
	// 불변성
    let mut f = 200; // consider making this binding mutable: `mut f`
    f = f + 34;
    println!("sum   : {}", f); // cannot assign twice to immutable variable
	```

### 상수
- const로 선언
	```rust
	const PI: f64 = 3.14159265359;
    let dist = 5.0;
    let size = PI * dist * dist;
    println!("area  : {}", size);
	```

### Shadowing
- 현재 범위에 있는 변수와, 바깥 범위에 있는 변수 모두 가릴(쉐도잉) 수 있음

	<img src="https://raw.githubusercontent.com/hugoMGSung/study-rust/main/Images/rust_0002.png" width="600">

[Back](https://github.com/hugoMGSung/study-rust/blob/main/README.md)