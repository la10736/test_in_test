pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::{any::Any, collections::HashMap, sync::Mutex};

    use rstest::rstest;

    use super::*;
    lazy_static::lazy_static! {
        static ref HASHMAP: Mutex<HashMap<String, Box<dyn Any + Send>>> = {
            Mutex::new(HashMap::new())
        };
    }

    fn tname() -> String {
        std::thread::current().name().unwrap().to_string()
    }

    fn add_data<T: Send + 'static>(data: T) {
        HASHMAP.lock().unwrap().insert(tname(), Box::new(data));
    }

    fn take_data<T: 'static + Send>() -> T {
        *HASHMAP
            .lock()
            .unwrap()
            .remove(&tname())
            .unwrap()
            .downcast()
            .unwrap()
    }

    #[rstest]
    fn thread_local(
        #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] a: u32,
        #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] b: u32,
        #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] c: u32,
        #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] d: u32,
    ) {
        struct Data {
            a: u32,
            b: u32,
            c: u32,
            d: u32,
        }

        thread_local! {
            static DATA: Mutex<Option<Data>> = Mutex::new(None);
        }
        {
            DATA.with(|r| {
                *r.lock().unwrap() = Some(Data { a, b, c, d });
            });
        }
        #[allow(unnameable_test_items)]
        {
            #[tokio::test]
            async fn it_works() {
                let Data { a, b, c, d } = DATA.with(|r| r.lock().unwrap().take().unwrap());
                let result = add(2, 2);
                assert_eq!(result, 4);
            }
            it_works()
        }
    }

    // #[rstest]
    // #[allow(unnameable_test_items)]
    // fn fast(
    //     #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] a: u32,
    //     #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] b: u32,
    //     #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] c: u32,
    //     #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] d: u32,
    // ) {
    //     struct Data {
    //         a: u32,
    //         b: u32,
    //         c: u32,
    //         d: u32,
    //     }
    //     add_data(Data { a, b, c, d });
    //     #[allow(unnameable_test_items)]
    //     {
    //         #[tokio::test]
    //         async fn it_works() {
    //             let Data { a, b, c, d } = take_data();
    //             let result = add(2, 2);
    //             assert_eq!(result, 4);
    //         }
    //         it_works()
    //     }
    // }

    // #[rstest]
    // #[tokio::test]
    // async fn slow(
    //     #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] a: u32,
    //     #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] b: u32,
    //     #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] c: u32,
    //     #[values(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] d: u32,
    // ) {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
