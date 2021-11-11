mod outermost{
    pub fn middle_function(){}
    fn middle_secret_function(){}
    mod inside{
        ::outermost::middle_secret_function!();
        pub fn inner_function(){}
        pub fn secret_function(){}
    }
}

fn try_me(){
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}