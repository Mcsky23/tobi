


pub struct Challenge {
    pub name: String,
    pub description: String, 
    pub category: ChallengeType,
}

enum ChallengeType {
    Web,
    Pwn,
    Crypto,
    Forensics,
    Reversing,
    Misc,
}