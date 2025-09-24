pub const BASIC_PROMPT: &str = "Write an insightful but concise Git commit message in a complete sentence in present tense for the
following diff without prefacing it with anything, the response must be in the language {locale} and must
NOT be longer than 74 characters. The sent text will be the differences between files, where deleted lines
are prefixed with a single minus sign and added lines are prefixed with a single plus sign.";
