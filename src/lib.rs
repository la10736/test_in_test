pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    // Not too much gain in compilation but it's harder to implement due generics 
    // definitions
    // // struct Data {
    // //     _a: u32,
    // //     _b: u32,
    // //     _c: u32,
    // //     _d: u32,
    // //     _e: u32,
    // // }

    // #[rstest]
    // fn thread_local(
    //     #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] _a: u32,
    //     #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] _b: u32,
    //     #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] _c: u32,
    //     #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] _d: u32,
    //     #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] _e: u32,
    // ) {
    //     struct Data {
    //         _a: u32,
    //         _b: u32,
    //         _c: u32,
    //         _d: u32,
    //         _e: u32,
    //     }
    //     thread_local! {
    //         static DATA: std::cell::RefCell<Option<Data>>  = std::cell::RefCell::new(None);
    //     }
    //     DATA.with(|r| r.replace(Some(Data { _a, _b, _c, _d, _e })));
    //     #[allow(unnameable_test_items)]
    //     {
    //         #[tokio::test]
    //         async fn it_works() {
    //             let Data { _a, _b, _c, _d, _e } = DATA.with(|r| r.replace(None)).unwrap();
    //             let result = add(2, 2);
    //             assert_eq!(result, 4);
    //         }
    //         it_works()
    //     }
    // }

    #[rstest]
    #[tokio::test]
    async fn slow(
        #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] _a: u32,
        #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] _b: u32,
        #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] _c: u32,
        #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] _d: u32,
        #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] _e: u32,
    ) {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
