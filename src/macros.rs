macro_rules! parameterized_test {
        ($func_name:ident;
            { $( $test_name:ident : ( $( $arg:expr ),* ) => $expected_result:expr ),* }
        ) => {
            mod $func_name {
                use super::*;
                $(
                    #[test]
                    fn $test_name() {
                        assert_eq!($expected_result, $func_name( $( $arg ),* ));
                    }
                )*
             }
        };

         // support for optional trailing comma
         ($func_name:ident;
            { $( $test_name:ident : ( $( $arg:expr ),* ) => $expected_result:expr ),+ ,}
         ) => {
            parameterized_test! {
                $func_name;
                { $( $test_name : ( $( $arg ),* ) => $expected_result),* }
            }
         };
    }
