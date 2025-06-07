use filename_range::filename_range;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref FILENAME_RANGE_TESTS: HashMap<&'static str, Vec<usize>> = {
        let mut tests = HashMap::new();
        tests.insert("src/Hiker_spec.re", vec![4, 9]);
        tests.insert("test/hiker_test.exs", vec![5, 10]);
        tests.insert("wibble/test/hiker_spec.rb", vec![12, 17]);
        tests.insert("hiker_steps.rb", vec![0, 5]);
        tests.insert("hiker_spec.rb", vec![0, 5]);
        tests.insert("test_hiker.rb", vec![5, 10]);
        tests.insert("test_hiker.py", vec![5, 10]);
        tests.insert("test_hiker.sh", vec![5, 10]);
        tests.insert("tests_hiker.sh", vec![6, 11]);
        tests.insert("test_hiker.coffee", vec![5, 10]);
        tests.insert("hiker_spec.coffee", vec![0, 5]);
        tests.insert("hikerTest.chpl", vec![0, 5]);
        tests.insert("hiker.tests.c", vec![0, 5]);
        tests.insert("hiker_tests.c", vec![0, 5]);
        tests.insert("hiker_test.c", vec![0, 5]);
        tests.insert("hiker_Test.c", vec![0, 5]);
        tests.insert("HikerTests.cpp", vec![0, 5]);
        tests.insert("hikerTests.cpp", vec![0, 5]);
        tests.insert("HikerTest.cs", vec![0, 5]);
        tests.insert("HikerTest.java", vec![0, 5]);
        tests.insert("DiamondTest.kt", vec![0, 7]);
        tests.insert("HikerTest.php", vec![0, 5]);
        tests.insert("hikerTest.js", vec![0, 5]);
        tests.insert("hiker-test.js", vec![0, 5]);
        tests.insert("hiker-spec.js", vec![0, 5]);
        tests.insert("hiker.test.js", vec![0, 5]);
        tests.insert("hiker.tests.ts", vec![0, 5]);
        tests.insert("hiker_tests.erl", vec![0, 5]);
        tests.insert("hiker_test.clj", vec![0, 5]);
        tests.insert("fizzBuzz_test.d", vec![0, 8]);
        tests.insert("hiker_test.go", vec![0, 5]);
        tests.insert("hiker.tests.R", vec![0, 5]);
        tests.insert("hikertests.swift", vec![0, 5]);
        tests.insert("HikerSpec.groovy", vec![0, 5]);
        tests.insert("hikerSpec.feature", vec![0, 5]);
        tests.insert("hiker.feature", vec![0, 5]);
        tests.insert("hiker.fun", vec![0, 5]);
        tests.insert("hiker.t", vec![0, 5]);
        tests.insert("hiker.plt", vec![0, 5]);
        tests.insert("prj/src/Hiker_bike_spec.re.bak", vec![8, 18]);
        tests.insert("src\\Hiker_spec.re", vec![4, 9]);
        tests.insert("hiker", vec![0, 5]);
        tests.insert("", vec![]);
        tests
    };
}

#[test]
fn test_filename_range() {
    for (filename, expected) in FILENAME_RANGE_TESTS.iter() {
        let result = filename_range(filename);
        assert_eq!(
            result, *expected,
            "filename_range({:?}) = {:?}; want {:?}",
            filename, result, expected
        );
    }
}
