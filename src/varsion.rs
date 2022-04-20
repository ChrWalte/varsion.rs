use regex::Regex;

// TODO: Update regex to show better understanding:
// let re = Regex::new(r"(?x)
//   (?P<y>\d{4}) # the year
//   -
//   (?P<m>\d{2}) # the month
//   -
//   (?P<d>\d{2}) # the day
// ").unwrap();
// example from: https://docs.rs/regex/latest/regex/#example-replacement-with-named-capture-groups

// stolen from: https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
const SEMVER_REGEX_RAW_STRING: &str = r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$";

pub struct Varsion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
    pub build: Option<String>,
}

impl Varsion {
    pub fn init(pre: Option<String>, build: Option<String>) -> Varsion {
        Varsion {
            major: 0,
            minor: 1,
            patch: 0,
            pre,
            build,
        }
    }

    pub fn update_major(mut self: Varsion, by_amount: i32) -> Varsion {
        self.major = (self.major as i32 + by_amount) as u32;
        self.minor = 0;
        self.patch = 0;
        self
    }

    pub fn update_minor(mut self: Varsion, by_amount: i32) -> Varsion {
        self.minor = (self.minor as i32 + by_amount) as u32;
        self.patch = 0;
        self
    }

    pub fn update_patch(mut self: Varsion, by_amount: i32) -> Varsion {
        self.patch = (self.patch as i32 + by_amount) as u32;
        self
    }

    pub fn update_pre(mut self: Varsion, to: String) -> Varsion {
        self.pre = Some(to);
        self
    }

    pub fn update_build(mut self: Varsion, to: String) -> Varsion {
        self.build = Some(to);
        self
    }

    pub fn to_string(self: Varsion) -> String {
        let pre = match self.pre {
            Some(pre) => format!("-{}", pre),
            None => String::from(""),
        };
        let build = match self.build {
            Some(build) => format!("+{}", build),
            None => String::from(""),
        };
        // major.minor.patch-pre+build
        format!(
            "{}.{}.{}{}{}",
            self.major, self.minor, self.patch, pre, build
        )
    }

    pub fn from_string(string: String) -> Varsion {
        let regex = Regex::new(SEMVER_REGEX_RAW_STRING).unwrap();
        let captures = regex.captures(&string).unwrap();

        // Example using println!("{:?}", captures);
        // Captures({0: Some("0.0.0"), 1: Some("0"), 2: Some("0"), 3: Some("0"), 4: None, 5: None})

        let major = match captures[1].parse::<u32>() {
            Ok(m) => m,
            Err(why) => panic!("failed to parse major: {}", why),
        };
        let minor = match captures[2].parse::<u32>() {
            Ok(m) => m,
            Err(why) => panic!("failed to parse minor: {}", why),
        };
        let patch = match captures[3].parse::<u32>() {
            Ok(p) => p,
            Err(why) => panic!("failed to parse patch: {}", why),
        };
        let pre = match captures.get(4) {
            Some(pre) => Some(pre.as_str().to_string()),
            None => None,
        };
        let build = match captures.get(5) {
            Some(build) => Some(build.as_str().to_string()),
            None => None,
        };

        Varsion {
            major,
            minor,
            patch,
            pre,
            build,
        }
    }

    pub fn valid_version(string: String) -> bool {
        let regex = Regex::new(SEMVER_REGEX_RAW_STRING).unwrap();
        regex.is_match(&string)
    }
}
