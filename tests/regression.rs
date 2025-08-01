#[test]
fn atmega328p() {
    let mut atdf = std::fs::File::open("tests/atmega328p.atdf").unwrap();
    let svd = atdf2svd::run_test(&mut atdf, vec![]);
    insta::assert_snapshot!(svd);
}

#[test]
fn atmega128rfa1() {
    let mut atdf = std::fs::File::open("tests/atmega128rfa1.atdf").unwrap();
    let svd = atdf2svd::run_test(&mut atdf, vec![]);
    insta::assert_snapshot!(svd);
}

#[test]
fn atxmega128a1() {
    let mut atdf = std::fs::File::open("tests/atxmega128a1.atdf").unwrap();
    let svd = atdf2svd::run_test(&mut atdf, vec![]);
    insta::assert_snapshot!(svd);
}

#[test]
fn attiny817() {
    let mut atdf = std::fs::File::open("tests/attiny817.atdf").unwrap();
    let svd = atdf2svd::run_test(&mut atdf, vec![]);
    insta::assert_snapshot!(svd);
}

#[test]
fn atmega4809() {
    let mut atdf = std::fs::File::open("tests/atmega4809.atdf").unwrap();
    let svd = atdf2svd::run_test(&mut atdf, vec![]);
    insta::assert_snapshot!(svd);
}

#[test]
fn atmega328p_unsafe_nofuse() {
    let mut atdf = std::fs::File::open("tests/atmega328p.atdf").unwrap();
    let svd = atdf2svd::run_test(
        &mut atdf,
        vec!["keep_unsafe_cpu_registers", "remove_fuse_and_lockbit"],
    );
    insta::assert_snapshot!(svd);
}
