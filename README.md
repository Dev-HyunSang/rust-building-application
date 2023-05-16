# rust-building-application
[Rust : 실제 애플리케이션 구축을 통한 Rust 완벽 정복](https://kmooc.udemy.com/course/rust-building-application)를 통해서 공부하고 있습니다.

## Table of Contents
- [rust-building-application](#rust-building-application)
  - [Table of Contents](#table-of-contents)
  - [Q\&A](#qa)
    - [Rust언어에서 mut가 뜻하는 게 뭐야?](#rust언어에서-mut가-뜻하는-게-뭐야)
    - [Rust언어에서 소유권 규칙을 알려줘.](#rust언어에서-소유권-규칙을-알려줘)

## Q&A
ChatGPT를 통해서! 공부하면서 궁금한 점을 해결하고 있습니다.

### Rust언어에서 mut가 뜻하는 게 뭐야?
Rust 언어에서 mut는 "mutable"을 나타내는 키워드입니다. 변수 선언 시 mut 키워드를 사용하면 해당 변수는 가변(mutable) 변수가 됩니다. 즉, 값을 변경할 수 있는 변수를 선언하는 것을 의미합니다.

Rust는 기본적으로 변수가 불변(immutable)으로 선언되어 있으며, 값을 변경하려면 변수를 가변으로 선언해야 합니다. 가변 변수를 사용하면 해당 변수의 값을 변경할 수 있으며, 이는 Rust의 소유권(ownership) 시스템과 빌더 패턴(builder pattern)의 일부와 관련이 있습니다.

다음은 mut를 사용하여 가변 변수를 선언하는 예시입니다:
```rust
fn main() {
    let mut x = 5; // 가변 변수 x 선언
    println!("x: {}", x); // x: 5

    x = 10; // 변수 x의 값을 변경
    println!("x: {}", x); // x: 10
}
```

### Rust언어에서 소유권 규칙을 알려줘.
위 예시에서 x 변수는 mut 키워드로 선언되었기 때문에 값을 변경할 수 있습니다. 첫 번째 println! 매크로 호출 이후에 x 값을 10으로 변경하고 두 번째 println! 매크로 호출에서 변경된 값이 출력됩니다.

Rust 언어에서는 소유권(ownership) 규칙이라는 중요한 개념을 가지고 있습니다. 이 규칙은 Rust의 메모리 안전성을 보장하고, 데이터 경합 문제나 댈러스틱 메모리 관리를 방지하는 데 도움을 줍니다. 소유권 규칙은 다음과 같이 요약할 수 있습니다:

각 값은 오직 하나의 소유자(Owner)만 가질 수 있습니다.
소유자가 범위(scope)를 벗어나면 해당 값은 자동으로 해제됩니다.
값은 오직 소유자를 통해서만 접근할 수 있습니다.
이 규칙을 통해 Rust는 다음과 같은 장점을 가집니다:

데이터 경합 문제를 컴파일 시간에 잡아냄: 동시에 여러 스레드에서 데이터에 접근하는 경우, Rust는 소유권 규칙을 통해 데이터 경합 문제를 사전에 방지할 수 있습니다.
댈러스틱 메모리 관리 방지: Rust는 소유권 규칙을 통해 메모리 할당과 해제를 정확하게 추적하여 댈러스틱 메모리 관리 문제를 방지합니다. 이를 통해 메모리 누수나 무효한 포인터 문제를 사전에 방지할 수 있습니다.
복사보다 이동(move)을 선호: 소유권 규칙은 값의 이동(move)을 강제함으로써 복사보다 효율적인 메모리 관리를 가능하게 합니다.
이 규칙은 Rust의 핵심 원칙이며, 개발자는 이를 이해하고 코드를 작성해야 합니다. Rust 컴파일러는 이러한 규칙을 검사하여 소유권 규칙을 준수하지 않는 코드를 컴파일 시간에 에러로 표시합니다.

---

[The Rust Programming Language - 소유권 이해하기](https://rinthel.github.io/rust-lang-book-ko/ch04-00-understanding-ownership.html)를 참고하면 좋아요!