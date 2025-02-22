
pub struct Keywords {
    pub keyword: String,
    pub value: String,
}

pub struct Args {
    pub positional: Vec<String>,
    pub flags: Vec<String>,
    pub keywords: Vec<Keywords>,
}

impl Args {
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Args, &'static str> {
        args.next();    // Drop the commands arg
        let mut ret_args = Args {
            positional: vec![],
            flags: vec![],
            keywords:  vec![],
        };

        while let Some(arg) = args.next() {
            match arg {
                keyword if keyword.starts_with("--") => {       // Keywork Arg
                    let next = args.next();
                    if let Some(value) = next {
                        ret_args.keywords.push(Keywords { keyword, value});
                    } else {
                        return Err("Keywork arguments (--{keyword}) should have a following value.");
                    }
                },
                flag if flag.starts_with("-") => {        // Flag
                    ret_args.flags.push(flag);
                }
                a => {          // Positional Arg
                    ret_args.positional.push(a);
                }
            }
        }

        Ok(ret_args)
    }
}

