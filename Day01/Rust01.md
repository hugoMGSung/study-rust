# study-rust
러스트 학습 리포지토리

## Day 01

### 개요
- 누가 만들었나?
	- 2006년 모질라 소속의 개발자 그레이던 호어(Graydon Hoare)가 개인 프로젝트로 시작했으나, 2009년 모질라가 참여하며 확대되었음
	- 2010년 일반에게 공개함
	- 모질라 정책에 따라 오픈소스로 공개

- 왜 만들었나?
	- 일단 모르겠다.
	- 하지만, 안전성, 속도, 동시성에 초점을 두고 형안전성 멀티패러다임 언어로,
	- 구글은 Go를 마이크로소프트는 Rust를 기존 C/C++으로 만들어진 시스템을 대체하고 있다.

- 지금 배우는 이유
	- 취미로...
	- 하지만, 나중에 어떻게 될지...

### 개발환경 구축
- Visual Studio Code에 Rust 개발환경 구성하기
	- Microsoft C++ Build Tools 설치
		https://visualstudio.microsoft.com/ko/visual-cpp-build-tools/ 에서 설치
	- Rust 설치
		https://www.rust-lang.org/tools/install 에서 설치

		- %USERPROFILE%\.cargo\bin 에 위치하므로, 시스템 등록정보에 Path 등록이 필요하면 구성할 것

		```shell
		> rustc --version
		rustc 1.75.0 (82e1608df 2023-12-21)
		```

- 프로젝트 생성

	- 콘솔에서 명령어 입력
		```shell
		> cargo new rust01(프로젝트명)
		```

	- Visual Studio Code를 위의 위치에서 오픈

	- 확장 프로그램 설치
		- rust-analyzer : 러스트 코드 분석
		- CodeLLDB : 러스트 디버깅

## 러스트 코드 실행
- 코드 실행
	1. 코드 작성 후 컴파일
	