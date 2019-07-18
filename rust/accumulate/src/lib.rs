pub fn map<F, I, O>(input: Vec<I>, mut function: F) -> Vec<O>
where
    F: FnMut(I) -> O,
{
    let mut output = Vec::with_capacity(input.len());
    for n in input {
        output.push(function(n))
    }
    output
}
