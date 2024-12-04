pub type Input<'a> = Vec<&'a [u8]>;

pub fn parse<'a>(input: &'a str) -> Input<'a> {
    input.lines().map(|l| l.as_bytes()).collect()
}
