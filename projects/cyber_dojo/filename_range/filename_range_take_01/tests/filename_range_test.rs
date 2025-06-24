use rstest::rstest;

use filename_range::filename_range;

#[rstest]
#[case::path_with_spec_suffix("src/Hiker_spec.re", vec![4, 9])]
#[case::path_with_test_suffix("test/hiker_test.exs", vec![5, 10])]
#[case::nested_path_with_spec_suffix("wibble/test/hiker_spec.rb", vec![12, 17])]
#[case::steps_suffix("hiker_steps.rb", vec![0, 5])]
#[case::spec_suffix("hiker_spec.rb", vec![0, 5])]
#[case::test_prefix("test_hiker.rb", vec![5, 10])]
#[case::test_prefix_python("test_hiker.py", vec![5, 10])]
#[case::test_prefix_shell("test_hiker.sh", vec![5, 10])]
#[case::tests_prefix("tests_hiker.sh", vec![6, 11])]
#[case::test_prefix_coffee("test_hiker.coffee", vec![5, 10])]
#[case::spec_suffix_coffee("hiker_spec.coffee", vec![0, 5])]
#[case::camel_case_test_suffix("hikerTest.chpl", vec![0, 5])]
#[case::dot_tests_suffix("hiker.tests.c", vec![0, 5])]
#[case::underscore_tests_suffix("hiker_tests.c", vec![0, 5])]
#[case::underscore_test_suffix("hiker_test.c", vec![0, 5])]
#[case::mixed_case_test_suffix("hiker_Test.c", vec![0, 5])]
#[case::pascal_case_tests_suffix("HikerTests.cpp", vec![0, 5])]
#[case::camel_case_tests_suffix("hikerTests.cpp", vec![0, 5])]
#[case::pascal_case_test_suffix_cs("HikerTest.cs", vec![0, 5])]
#[case::pascal_case_test_suffix_java("HikerTest.java", vec![0, 5])]
#[case::different_name_with_test_suffix("DiamondTest.kt", vec![0, 7])]
#[case::pascal_case_test_suffix_php("HikerTest.php", vec![0, 5])]
#[case::camel_case_test_suffix_js("hikerTest.js", vec![0, 5])]
#[case::hyphen_test_suffix("hiker-test.js", vec![0, 5])]
#[case::hyphen_spec_suffix("hiker-spec.js", vec![0, 5])]
#[case::dot_test_suffix("hiker.test.js", vec![0, 5])]
#[case::dot_tests_suffix_ts("hiker.tests.ts", vec![0, 5])]
#[case::underscore_tests_suffix_erl("hiker_tests.erl", vec![0, 5])]
#[case::underscore_test_suffix_clj("hiker_test.clj", vec![0, 5])]
#[case::different_name_with_test_suffix_d("fizzBuzz_test.d", vec![0, 8])]
#[case::underscore_test_suffix_go("hiker_test.go", vec![0, 5])]
#[case::dot_tests_suffix_r("hiker.tests.R", vec![0, 5])]
#[case::tests_suffix_no_separator_swift("hikertests.swift", vec![0, 5])]
#[case::pascal_case_spec_suffix_groovy("HikerSpec.groovy", vec![0, 5])]
#[case::camel_case_spec_suffix_feature("hikerSpec.feature", vec![0, 5])]
#[case::simple_feature_extension("hiker.feature", vec![0, 5])]
#[case::simple_fun_extension("hiker.fun", vec![0, 5])]
#[case::simple_t_extension("hiker.t", vec![0, 5])]
#[case::simple_plt_extension("hiker.plt", vec![0, 5])]
#[case::complex_path_with_multiple_words("prj/src/Hiker_bike_spec.re.bak", vec![8, 18])]
#[case::windows_path_with_spec_suffix("src\\Hiker_spec.re", vec![4, 9])]
#[case::no_extension_or_suffix("hiker", vec![0, 5])]
#[case::empty_filename("", vec![])]
fn test_filename_range(#[case] filename: &str, #[case] expected: Vec<usize>) {
    let result = filename_range(filename);
    assert_eq!(
        result, expected,
        "filename_range({:?}) = {:?}; want {:?}",
        filename, result, expected
    );
}
