

// * 모든 데이터들은 메모리 주소를 갖습니다.
//   이 주소는 메모리에 있는 데이터들의 물리적 위치를 결정하는 데 사용됩니다.
// * 오프셋은 인접 주소에 접근하는 데 사용될 수 있습니다
//   오프셋은 인덱스 또는 복수형으로 indices 로 불리기도 합니다

// 보통 오프셋은 프로그램을 컴파일할 때, 프로그래밍 언어에 의해 자동으로 계산됩니다.

// Stack
//
// * 데이터는 스택 메모리의 한 부분이며 순차적으로 배치됩니다.
// * 스택은 제한된 공간을 가지며, 처음 프로그램이 실행될 때 OS에 의해 할당됩니다
// * 변수는 항상 대부분 스택 메모리에 저장된다.
// * 아주 빠르게 데이터에 접근하기 위해 오프셋을 사용합니다

// Heap
//
// * 힙 메모리에서는 데이터가 알고리즘적으로 배치됩니다.
//   그래서 스택보다 약간 느리게 처리됩니다.
// * 하지만 힙 공간은 무제한입니다. ( 램이나 디스크처럼 물리적인 제한은 적용됩니다)
// * 데이터 접근을 위해 포인터를 사용합니다.
//   포인터는 고징된 크기이며,
//   러스트에서 포인터를 나타내는 타입으로는 usize와 isize를 사용합니다
// * 일반적으로 힙에 저장되는 데이터 구조는 자동으로 벡터와 해시맵 입니다.
//
// 힙 메모리에 있는 데이터는 임의의 위치에 흩어져 있습니다.
// 이런 위치는 시스템과 프로그램의 메모리 사용에 따라 동적으로 계산됩니다.
// 위의 이유때문에 힙은 처리하는 데 좀더 시간이 걸립니다.
// 스택보다 약간 더 느린 또 다른 이유는 힙 메모리는 OS에서 요청해야 하기 때문입니다.
// 하지만 일단 메모리가 할당되면, 아주 빠르게 처리되고, 데이터에 접근하기 위해 포인터를 사용합니다.
// 그래서 세 번째 아이템에 접근하기 위해 1,2 를 더하는 대신, 정확한 주소를 나타내는 포인터를 사용해야 할 것 입니다.
// 그래서 이렇게 바로 특정 위치를 가리킬 수 있습니다.
// 포인터는 간단히 메모리 주소입니다.
// 포인터가 있으면, 포인터가 가리키는 위치의 데이터에 접근할 수 있습니다.
// Heap Example
// 열거형 Answer
enum Answer {
    Yes,
    No,
}

fn main() {
    // 변수 yes를 생성해서 enum을 할당했습니다.
    // 현재는 스택에 존재.
    let yes = Answer::Yes;
    // 힙에 저장하기 위해 Box를 생성 ( Box는 포인터, 메모리 주소입니다 )
    let yes_heap: Box<Answer> = Box::new(yes);
    // 힙에 접근하기 위해 '*'와 함께 변수명을 사용합니다.
    let yes_stack = *yes_heap;

    // 스택에 있는 값 yes를 가져와서 Box에 넣고
    // Box는 힙으로 이동합니다
    // `yes_heap` 변수 자체는 스택에 저장되어 있습니다.
    // 하지만 실제 데이터는 힙의 Box에 저장되어 있습니다.
    // `yes_heap` 의 데이터를 스택으로 다시 가져오려면 heap 밖으로 가져오기 위해
    // '*'를 사용해서 스택으로 가져와야 합니다.
}

// Recap

// Stack
// * 스택 메모리는 순차적으로 배치됩니다.
// * 프로그램에서 변수를 할당하는 데 사용되고
// * 제한된 크기를 갖습니다.

// Heap
// * 힙 메모리는 알고리즘 적으로 배치됩니다.
// * 대용량 데이터를 위해 사용됩니다.
// * 힙은 크기에 제한이 없지만, 장치의 물리적은 크기에 의해 제한됩니다.