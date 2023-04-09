pub mod commandline {

    pub fn get_security(args: &Vec<String>) -> String {
        if args.len() < 2 {
            panic!("No security specified - Stopping...") // TODO: Add error handling
        }

        let security = &args[1];
        String::from(security)
    }
}
