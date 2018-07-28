pub fn s<T>(input: T)
    -> String
where
    T: Into<String>
{
    input.into()
}