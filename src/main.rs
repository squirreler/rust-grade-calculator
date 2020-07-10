fn main() {
    let x: String = return_letter_grade_given_numeric_score(86);
    println!("{}", x);
    //let y = tell_if_str_is_positive(1);
    //println!("{}", y);
}

fn return_letter_grade_given_numeric_score(score: i32) -> String {
    let letter_grade;
    if score < 60 {
        letter_grade = String::from("F");
    } else if score >= 60 && score < 69 {
        letter_grade = String::from("D");
    } else if score >= 70 && score <= 79 {
        letter_grade = String::from("C");
    } else if score >= 80 && score <= 89 {
        letter_grade = String::from("B");
    } else if score >= 90 && score <= 95 {
        letter_grade = String::from("A");
    } else if score >= 96 && score <= 100 {
        letter_grade = String::from("A+");
    } else {
        letter_grade = String::from("This is not a grade");
    }
    println!("{}", letter_grade);
    letter_grade

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score59_returns_f() {
        assert_eq!(return_letter_grade_given_numeric_score(59), "F");
    }
    #[test]
    fn score62_returns_f() {
        assert_eq!(return_letter_grade_given_numeric_score(62), "D");
    }
    #[test]
    fn score72_returns_f() {
        assert_eq!(return_letter_grade_given_numeric_score(72), "C");
    }
    #[test]
    fn score82_returns_f() {
        assert_eq!(return_letter_grade_given_numeric_score(82), "B");
    }
    #[test]
    fn score92_returns_f() {
        assert_eq!(return_letter_grade_given_numeric_score(92), "A");
    }
    #[test]
    fn score96_returns_f() {
        assert_eq!(return_letter_grade_given_numeric_score(96), "A+");
    }
}

 fn tell_if_str_is_positive(pos_or_neg_num: i32) -> String {
     let z = &pos_or_neg_num[..0];
     let x = if z == 1 { "yes" } else { "no" };
    
    
     x.to_string()
} 