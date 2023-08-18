fn split_grades(grades: Vec<&'static str>) -> (Vec<&'static str>, Vec<&'static str>) {
    let mut above_d = Vec::new();
    let mut below_d = Vec::new();

    if grades.is_empty() {
        return (above_d,below_d)
    }
    else {
        for i in grades {
            match i {
                "A+"|"A"|"B"|"C" => above_d.push(i),
                "D"|"F" => below_d.push(i),
                _ => {}
            }
        }
        return (above_d,below_d);
    }
}

fn split_score(score: &[i32]) -> (Vec<(&'static str, i32)>, Vec<(&'static str, i32)>) {
    let mut above_d = Vec::new();
    let mut below_d = Vec::new();

    for &i in score {
        if i >= 81 && i < 95 {
            above_d.push(("A",i))
        }
        else if i >= 95 && i < 101 {
            above_d.push(("A+",i))
        }
        else if i >= 71 && i < 81 {
            above_d.push(("B",i))
        }
        else if i >= 61 && i < 71 {
            above_d.push(("C",i))
        }
        else if i >= 50 && i < 61 {
            below_d.push(("D",i))
        }
        else if i >= 0 && i < 50 {
            below_d.push(("F",i))
        };
    }
    (above_d,below_d)
}

#[test]
fn split_grades_test() {
    assert_eq!(split_grades(["B", "F", "A+", "D", "C"].to_vec()),(["B", "A+", "C"].to_vec(), ["F", "D"].to_vec()));
    assert_eq!(split_grades([].to_vec()),([].to_vec(),[].to_vec()));
}

#[test]
fn split_score_test() {
    assert_eq!(split_score(&[75, 42, 98, 54, 63]),([("B",75), ("A+", 98), ("C", 63)].to_vec(), [("F", 42), ("D", 54)].to_vec()));
}