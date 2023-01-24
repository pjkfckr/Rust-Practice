

struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages);
}

fn display_rating(book: Book) {
    println!("rating = {:?}", book.rating);
}

fn main() {
    let book = Book {
        pages: 5,
        rating: 9
    };
    display_page_count(&book);
    display_rating(book);
    // 데이터 구조체가 몇 mb에 달할 정도로 컸다면 이 구조체의 쇼유권을 다른 함수로 옮기면
    // 함수를 사용할 때마다 이 데이터 전체를 복사해야 합니다
    // 그러나 참조를 사용하면 데이터가 한 곳에 그대로 머물러 있고 단순히
    // 참조한 뒤 돌려 받는 것이므로 훨씬 빠르게 동작합니다
    // 소유권과 참조와 이동을 잘 사용하면 성능을 크게 향상시킬 수 있습니다
}


