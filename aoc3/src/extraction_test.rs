mod multiplication_pairs {
    use crate::extraction::{extract_multiplication_pairs_from_text, MultiplicationPair};

    #[test]
    fn extract_single_multiplication_pair_test() {
        let text = "mul(1,2)";

        let expected_pair = MultiplicationPair(1, 2);

        let multiplication_pairs = extract_multiplication_pairs_from_text(text);

        assert_eq!(expected_pair, multiplication_pairs[0]);
    }

    #[test]
    fn extract_multiple_multiplication_pairs_test() {
        let text = "mul(1,2)mul(3,4)";
        let expected_pairs = vec![MultiplicationPair(1, 2), MultiplicationPair(3, 4)];

        let multiplication_pairs = extract_multiplication_pairs_from_text(text);

        assert_eq!(expected_pairs, multiplication_pairs);
    }

    #[test]
    fn extract_multiple_multiplication_pairs_with_garbage_text_test() {
        let text = "mul(1,2)wiofnjavnkjvnmul(3,4)";
        let expected_pairs = vec![MultiplicationPair(1, 2), MultiplicationPair(3, 4)];

        let multiplication_pairs = extract_multiplication_pairs_from_text(text);

        assert_eq!(expected_pairs, multiplication_pairs);
    }
}

mod activated_multiplications {
    use crate::extraction::extract_activated_multiplications;

    #[test]
    fn extract_multiplication_without_activation_or_deactivation() {
        let text = "mul(1,2)";
        let extracted_multiplications = extract_activated_multiplications(text);

        assert_eq!(text, extracted_multiplications);
    }

    #[test]
    fn dont_extract_multiplication_after_deactivation() {
        let text = "don't()mul(1,2)do()";

        let extracted_multiplications = extract_activated_multiplications(text);

        let expected_multiplications = "";
        assert_eq!(expected_multiplications, extracted_multiplications);
    }

    #[test]
    fn extract_multiplication_after_deactivation_and_activation() {
        let text = "don't()mul(1,2)do()mul(1,3)";

        let extracted_multiplications = extract_activated_multiplications(text);

        let expected_multiplications = "mul(1,3)";
        assert_eq!(expected_multiplications, extracted_multiplications);
    }

    #[test]
    fn extract_multiplication_with_newline() {
        let text = "don't()\nmul(1,2)do()";

        let extracted_multiplications = extract_activated_multiplications(text);

        let expected_multiplications = "";
        assert_eq!(expected_multiplications, extracted_multiplications);
    }
}
