use std::fs;

use assert_cmd::Command;

// ToDo
// - [ ] 指定したクエリの値を取り出す
// - [ ] コマンドにURLが渡されなかった場合はエラー終了してUSAGEを表示する
// - [ ] 指定したパスの値を取り出す
// - [ ] ホスト名を取り出す
// - [ ] プロトコルを取り出す
// - [ ] クエリの指定を複数できるようにする
// - [ ] パスの指定を複数できるようにする
// - [ ] パース結果をJSONで出力できるようにする
// - [ ] URLエンコードに対応する

fn run(args: &[&str], expected_file: &str) -> anyhow::Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("purl")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn query() -> anyhow::Result<()> {
    run(&["https://example.com/?q=foo"], "tests/expected/query.txt")
}

#[test]
fn query2() -> anyhow::Result<()> {
    Command::cargo_bin("purl")?
        .write_stdin("https://purl.com/foo?q=bar")
        .assert()
        .success()
        .stdout("bar\n");
    Ok(())
}
