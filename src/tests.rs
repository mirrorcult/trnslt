use crate::*;

// Test various language translating to / from
#[test]
fn test_translation() {
    // Eng -> spanish
    assert_eq!(
        translate("To be or not to be", "en", "es"),
        "Ser o no ser"
    );

    // Spanish -> turkish
    assert_eq!(
        translate("Ser o no ser", "es", "tr"),
        "Olmak ya da olmamak"
    );

    // Arabic -> icelandic
    assert_eq!(
        translate("Olmak ya da olmamak", "tr", "is"),
        "Að vera eða ekki vera"
    )
}
