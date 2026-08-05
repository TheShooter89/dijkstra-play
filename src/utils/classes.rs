use std::collections::HashSet;

pub fn merge_classes(classes: &str, additional_classes: Option<&str>) -> String {
    let seen: HashSet<&str> = classes.split_whitespace().collect();

    classes
        .split_whitespace()
        .chain(
            additional_classes
                .into_iter()
                .flat_map(str::split_whitespace)
                .filter(|cl| !seen.contains(cl)),
        )
        .collect::<Vec<&str>>()
        .join(" ")
}

pub fn concat_with_optional_condition(
    base: &str,
    condition: Option<bool>,
    to_concat: &str,
) -> String {
    match condition.unwrap_or(true) {
        true => [base, to_concat].join(" "),
        // true => format!("{base} {to_concat}"),
        false => base.into(),
    }
}

#[cfg(test)]
mod test {
    use crate::utils::classes::{concat_with_optional_condition, merge_classes};

    #[test]
    fn test_util_classes() {
        let base_classes = "btn btn-white";
        let additional_classes = "mx-6 my-2 hover:shadow";

        let expected_result: String = "btn btn-white mx-6 my-2 hover:shadow".into();

        let result = merge_classes(base_classes, Some(additional_classes));

        println!("result: {}", result);

        assert_eq!(result, expected_result);

        // assert_eq!(4, 2)
    }

    #[test]
    fn test_util_classes_with_duplicates() {
        let base_classes = "btn pb-0 btn-white mx-6";
        let additional_classes = "mx-6 my-2 pb-0 hover:shadow";

        let expected_result: String = "btn pb-0 btn-white mx-6 my-2 hover:shadow".into();

        let result = merge_classes(base_classes, Some(additional_classes));

        println!("result: {}", result);

        assert_eq!(result, expected_result);

        // assert_eq!(4, 2)
    }

    #[test]
    fn test_util_concat_with_optional_condition() {
        let base_classes = "btn btn-white";
        let additional_classes = "mx-6 my-2";

        // INFO: CASE #1: condition has been specified and it's TRUE
        let optional_condition = Some(true);

        let expected_result: String = "btn btn-white mx-6 my-2".into();

        let result = concat_with_optional_condition(
            //
            base_classes,
            optional_condition,
            additional_classes,
        );

        println!("result: {}", result);

        assert_eq!(result, expected_result);

        // INFO: CASE #1: condition has been specified and it's FALSE
        let optional_condition = Some(false);

        let expected_result: String = "btn btn-white".into();

        let result = concat_with_optional_condition(
            //
            base_classes,
            optional_condition,
            additional_classes,
        );

        println!("result: {}", result);

        assert_eq!(result, expected_result);

        // INFO: CASE #1: condition is None or has not been specified
        //                and should be treated as Same(true)
        let optional_condition = None;

        let expected_result: String = "btn btn-white mx-6 my-2".into();

        let result = concat_with_optional_condition(
            //
            base_classes,
            optional_condition,
            additional_classes,
        );

        println!("result: {}", result);

        assert_eq!(result, expected_result);

        // assert_eq!(4, 2)
    }
}
