use crate::*;

// Test various language translating to / from
#[test]
fn test_translation() {
    // Eng -> spanish
    assert_eq!(
        translate("To be or not to be", "en", "es"),
        "Ser o no ser"
    );

    // Spanish -> arabic
    assert_eq!(
        translate("Ser o no ser", "es", "ar"),
        "أن تكون أو لا تكون"
    );

    // Arabic -> icelandic
    assert_eq!(
        translate("أن تكون أو لا تكون", "ar", "is"),
        "Að vera eða ekki vera"
    )
}
