
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "main_menu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned())
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

// ()는 유닛타입이라고 불린다.
// ? 를 사용하면 Result값을 반환하는 대신, 물음표가 자동으로 match 동작을 실행하게 됩니다.
// 물음표 연산자를 사용하면 여러 함수를 연쇄적으로 호출할 수 있습니다.
// 함수가 실패할 가능성이 있는 경우에도요
// 그 중 하나가 실패하면 물음표 연산자가 알아서 오류를 반환할 테니까요,
// 모든 결과에서 match 문을 추가할 필요가 없습니다
fn pick_choice(input: &str) -> Result<(), String> {
    let choice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}

fn main() {
    pick_choice("main_meau");
}