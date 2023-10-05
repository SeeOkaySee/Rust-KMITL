fn vflip(img: &[String]) -> Vec<String> {
    let mut flipped_img = Vec::new();
    for line in img.iter().rev() {
        flipped_img.push(line.to_string());
    }
    flipped_img
}

fn hflip(img: &[String]) -> Vec<String> {
    let mut flipped_img = Vec::new();
    for line in img {
        let mut reversed_line = String::new();
        for character in line.chars().rev() {
            reversed_line.push(character);
        }
        flipped_img.push(reversed_line);
    }
    flipped_img
}

fn vcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut concatenated_img = img1.to_vec();
    concatenated_img.extend_from_slice(img2);
    concatenated_img
}

fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
    if img1.is_empty() {
        return img2.to_vec();
    }
    if img2.is_empty() {
        return img1.to_vec();
    }

    let mut concatenated_img = Vec::new();

    for (line1, line2) in img1.iter().zip(img2.iter()) {
        let mut new_line = line1.clone();
        new_line.push_str(" ");
        new_line.push_str(line2);
        concatenated_img.push(new_line);
    }

    if img1.len() > img2.len() {
        for line in img1.iter().skip(img2.len()) {
            concatenated_img.push(line.clone());
        }
    } else if img2.len() > img1.len() {
        for line in img2.iter().skip(img1.len()) {
            concatenated_img.push(line.clone());
        }
    }

    concatenated_img
}

#[test]
fn test_img_flip() {
    let emp = ["".to_string(); 0];
    assert_eq!(vflip(&emp), [""; 0]);
    assert_eq!(hflip(&emp), [""; 0]);
    let data = [
    "<--",
    "#####",
    "<=="
    ].map(|v| v.to_string());
    assert_eq!(
        vflip(&data), [
        "<==",
        "#####",
        "<--"
        ]);
    assert_eq!(
        hflip(&data), [
        "--<",
        "#####",
        "==<"
        ]);
    }

#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    assert_eq!(hcat(&emp, &emp), [""; 0]);

    let data = [
        "<--",
        "#####",
        "<=="
    ].iter()
    .map(|v| v.to_string())
    .collect::<Vec<String>>();

    assert_eq!(vcat(&emp, &data), data.clone());
    assert_eq!(vcat(&data, &emp), data.clone());

    assert_eq!(
        vcat(&data, &data),
        [
            "<--",
            "#####",
            "<==",
            "<--",
            "#####",
            "<=="
        ].iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
    );

    assert_eq!(
        hcat(&data, &data[..2]),
        [
            "<-- <--",
            "##### #####",
            "<==",
        ].iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
    );

    assert_eq!(
        hcat(&data[..2], &data),
        [
            "<-- <--",
            "##### #####",
            "<==",
        ].iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
    );
}