use {
    rstest::*,
    tokio::process::Command,
};

#[rstest(
    count,
    case::coun_1(10),
    case::coun_1(14),
)]
async fn distrinute(count: u64) {
    let output = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .output().await.unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    println!("STDOUT: {}", stdout);
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");

    assert_eq!(count, 10);


    //println!("distribute {}", count);
}