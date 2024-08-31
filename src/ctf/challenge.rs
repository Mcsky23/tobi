


pub struct Challenge {
    pub name: String,
    pub category: ChallengeType,
    pub flag: String,
}

enum ChallengeType {
    Web,
    Pwn,
    Crypto,
    Forensics,
    Reversing,
    Misc,
}

impl Challenge {
    pub fn new(name: String, category: String, flag: String) -> Self {
        Challenge {
            name,
            category: match category.as_str() {
                "web" => ChallengeType::Web,
                "pwn" => ChallengeType::Pwn,
                "crypto" => ChallengeType::Crypto,
                "forensics" => ChallengeType::Forensics,
                "reversing" => ChallengeType::Reversing,
                "misc" => ChallengeType::Misc,
                _ => ChallengeType::Misc,
            },
            flag: flag,
        }
    }
}