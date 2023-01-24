

// String and &str

// 일반적으로 사용되는 두가지 타입인 String 과 &str이 있습니다

// String 타입은 소유 데이터 타입으로 이것이 곧 중요한 차이가 됩니다
// 그리고 str에 대한 참조인 &str이 있습니다. 이것은 차용된 문자열 슬라이스 입니다

// struct에 문자열 데이터를 저장하고 싶다면 소유 데이터 타입인 String을 사용해야 합니다
// struct에는 슬라이스를 저장할 수 없습니다. 아짂까지는 말이죠

// 함수에 문자열 데이터를 제공하려 한다면 문자열 슬라이스를 사용하는 것이 좋습니다 더 효율적이죠


// Example - Pass to function
fn print_it(data: &str) {
    println!("{:?}", data);
}

// Example - Will not work
struct Employee {
    // name: &str,
    name: String,
}

fn main() {
    print_it("a string slice");
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");
    print_it(&owned_string);
    print_it(&another_owned);

    // Struct에 차용된 데이터를 저장하려고 하면 컴파일이 되지 않습니다
    // 그 이유는 struct가 그 범위 끝에서 드랍 될 때 struct 자체의 메모리 정리를
    //  담당하기 때문입니다
    // 그렇지만 위의 Employee name에 차용된 메모리 &str이 있기때문에 struct는 이를 정리할 수가 없습니다
    // 소유권이 없기때문이죠
    // String 타입으로 변경하여 struct 내에서 문자열 데이터를 저장할 수 있도록 변경하면 가능합니다
    let emp_name = "Jayson".to_owned();
    let emp_name = String::from("Jayson");
    let emp = Employee {
        name: emp_name
    };
}

// Recap
// String은 자동적으로 차용됩니다. 따라서 .to_owned() 또는 String::from()을
// 사용하여 문자열 슬라이스의 복사본을 만들고
// struct에서는 소유 데이터 타입 String을 사용해 문자열을 저장합니다