fn main() {
    let grade = Some("A+");
    let mut grades = vec!["B-", "C+", "D"];

    // if let Some(g) = grade {
    //     grades.push(g)
    // }
    // 扩展数组, Option实现了IntoIterator
    grades.extend(grade);

    println!("{grades:?}");

    let grades = vec!["B-", "C+", "D"];
    // 扩展迭代器
    for g in grades.iter().chain(grade.iter()) {
        println!("{g}");
    }

    let grades = vec![Some("A+"), None, Some("B-"), None];
    // 扁平化：拿掉外层包装
    let grades: Vec<&str> = grades.into_iter().flatten().collect();
    println!("{grades:?}");

    let grades = ["3.8", "B+", "4.0", "A", "2.7"];
    let grades: Vec<f32> = grades.iter().filter_map(|s| s.parse().ok()).collect();
    println!("{grades:?}");
}
