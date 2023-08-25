//! This test is primarily to make sure that macros_rules for From traits are correct.

use stabilityai::types::TextPrompts;
fn text_prompts<T>(input: T) -> TextPrompts
where
    TextPrompts: From<T>,
{
    input.into()
}

#[test]
fn create_text_prompts() {
    let prompt = "This is &str prompt";
    let _ = text_prompts(prompt);

    let prompt = "This is String".to_string();
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);

    let prompt = vec!["This is first", "This is second"];
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);

    let prompt = vec!["First string".to_string(), "Second string".to_string()];
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);

    let first = "First".to_string();
    let second = "Second".to_string();
    let prompt = vec![&first, &second];
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);

    let prompt = ["first", "second"];
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);

    let prompt = ["first".to_string(), "second".to_string()];
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);

    let first = "First".to_string();
    let second = "Second".to_string();
    let prompt = [&first, &second];
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);
}

#[test]
fn create_text_prompts_with_weights() {
    let prompt = ("This is &str prompt", 1.0);
    let _ = text_prompts(prompt);

    let prompt = ("This is String".to_string(), 1.0);
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);

    let prompt = vec![("This is first", 0.2), ("This is second", 0.3)];
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);

    let prompt = vec![
        ("First string".to_string(), 0.1),
        ("Second string".to_string(), 0.2),
    ];
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);

    let first = "First".to_string();
    let second = "Second".to_string();
    let prompt = vec![(&first, 0.1), (&second, 0.22)];
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);

    let prompt = [("first", 0.11), ("second", 0.22)];
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);

    let prompt = [("first".to_string(), 0.12), ("second".to_string(), 0.12)];
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);

    let first = "First".to_string();
    let second = "Second".to_string();
    let prompt = [(&first, 0.111), (&second, 0.222)];
    let _ = text_prompts(&prompt);
    let _ = text_prompts(prompt);
}
