mod outermost {
    pub fn middle_function() {}
    pub fn middle_secret_function() {}
    pub mod inside {
        pub fn inner_function() {
            ::outermost::middle_secret_function();
        }
        pub fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
