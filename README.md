# echo_rs

***echo_rs*** is a simple echo program written in Rust, following the OpenBSD version of echo (https://man.openbsd.org/echo.1).

It takes input text and outputs the text with an optional trailing newline.

## Usage

```bash
echo_rs [FLAGS] <TEXT>...
```

## Arguments
    <TEXT>: The input text to be echoed (required, accepts one or more arguments). Multiple words should be separated by spaces.

## Flags
    -n, --omit_newline: Omits the trailing newline from the output.

## Examples

###### Echoing a single word
```bash
$ echo_rs Hello!
Hello!
```
###### Echoing multiple words
```bash
$ echo_rs Hello, world!
Hello, world!
```
###### Omitting the trailing newline
```bash
$ echo_rs -n Hello, world!
Hello, world%
```

***echo_rs*** is designed to maintain compatibility with the OpenBSD version of ***echo***. 
The provided tests help ensure that the output of ***echo_rs*** matches the expected output of OpenBSD's ***echo***.
To verify the consistency between the two utilities, a bash script is employed to generate and save the output from OpenBSD's ***echo*** to a file.

Subsequently, the output of ***echo_rs*** is compared to the saved output from OpenBSD's ***echo*** to confirm their similarity.

###### output.sh
```bash
#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello there" > $OUTDIR/hello1.txt
echo "Hello"  "there" > $OUTDIR/hello2.txt
echo -n "Hello there" > $OUTDIR/hello1.n.txt
echo -n "Hello" "there" > $OUTDIR/hello2.n.txt
```

###### cli.sh (testing file)
```rust
#[test]
fn matches_echos_output1() -> TestResult {
    let outfile = "hello1.txt";

    run(&["Hello there"], outfile)
}

#[test]
fn matches_echos_output2() -> TestResult {
    let outfile = "hello2.txt";

    run(&["Hello", "there"], outfile)
}

#[test]
fn matches_echos_output1_no_newline() -> TestResult {
    let outfile = "hello1.n.txt";

    run(&["Hello", "there", "-n"], outfile)
}

#[test]
fn matches_echos_output2_no_newline() -> TestResult {
    let outfile = "hello2.n.txt";

    run(&["Hello", "there", "-n"], outfile)
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let outfile = format!("tests/expected/{}", expected_file);
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echo_rs")?;

    cmd.args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
```