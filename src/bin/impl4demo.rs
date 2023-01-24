
struct Temperature {
    degrees_f: f64
}


impl Temperature {
    // self를 참조하면 컴파일러에서는 이 함수를 Temperature 형의
    // 변수로부터 호출할 수 있음을 알게됩니다
    fn show_temp(&self) {
        println!("{:?} degrees F", self.degrees_f);
    }

    // self와 Self 의 차이는
    // self의 경우 프로그램의 어딘가에서 이미 만들어진 Temperature를 의미합니다
    // Self는 아직 만들어지지 않아 새로 만드는 경우나 Temperature를 이름으로 참조할 때 쓰입니다
    fn freezing() -> Self {
        Self {
            degrees_f: 32.0,
        }
    }

    fn boiling() -> Self {
        Self {
            degrees_f: 212.0
        }
    }
}

fn main() {
    let hot = Temperature { degrees_f: 99.9 };
    hot.show_temp();
    let freezing = Temperature::freezing();
    freezing.show_temp();
    let boiling = Temperature::boiling();
    boiling.show_temp();
}