// for loop method
fn make_grades(v: &[i32]) -> Vec<&str> {
    let mut b = Vec::new();
    for &score in v {
        let grade = if score >= 95 && score < 101 {
            "Excellent with A+"
        } else if score >= 81 && score < 95 {
            "A"
        } else if score >= 71 && score < 81 {
            "B"
        } else if score >= 61 && score < 71 {
            "C"
        } else if score >= 50 && score < 61 {
            "D"
        } else if score >= 0 && score < 50 {
            "Failed with F"
        } else {
            "Invalid score"
        };
        b.push(grade);
    }
    b
}

// recursion method
fn make_grades_recur(v: &[i32], mut b: &mut Vec<&str>) {
    if v.is_empty() {
        return;
    }

    let score = v[0];
    let grade = if score >= 95 && score < 101 {
        "Excellent with A+"
    } else if score >= 81 && score < 95 {
        "A"
    } else if score >= 71 && score < 81 {
        "B"
    } else if score >= 61 && score < 71 {
        "C"
    } else if score >= 50 && score < 61 {
        "D"
    } else if score >= 0 && score < 50 {
        "Failed with F"
    } else {
        "Invalid score"
    };
    b.push(grade);

    make_grades_recur(&v[1..], &mut b);
}

#[test]
fn make_grades_test() {
    assert_eq!(make_grades(&mut[50,61,71,81,100]),["D","C","B","A","Excellent with A+"]);
    assert_eq!(make_grades(&mut[1000]),["Invalid score"]);
}

#[test]
fn test_make_grades_recur() {
    let mut grades = Vec::new();
    let scores_multiple: [i32; 6] = [92, 78, 60, 45, 101, 55];
    grades.clear();
    make_grades_recur(&scores_multiple, &mut grades);
    assert_eq!(grades, vec!["A", "B", "D", "Failed with F", "Invalid score", "D"]);
}