pub mod ddb;
pub mod s3;
pub mod lambda;
pub mod cognito;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
