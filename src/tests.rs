use strum::VariantArray;

use super::*;

#[test]
fn from_u32_strict() {
    assert_eq!(0_u32.in_toggle::<true>(), Ok(Toggle::Disable));
    assert_eq!(1_u32.in_toggle::<true>(), Ok(Toggle::Enable));
    assert!(2_u32.in_toggle::<true>().is_err());
    assert!(u32::MAX.in_toggle::<true>().is_err());
}

#[test]
fn from_u32_relaxed() {
    assert_eq!(0_u32.in_toggle::<false>(), Ok(Toggle::Disable));
    assert_eq!(1_u32.in_toggle::<false>(), Ok(Toggle::Enable));
    assert_eq!(2_u32.in_toggle::<false>(), Ok(Toggle::Enable));
    assert_eq!(u32::MAX.in_toggle::<false>(), Ok(Toggle::Enable));
}

#[test]
fn from_char_strict() {
    assert_eq!('0'.in_toggle::<true>(), Ok(Toggle::Disable));
    assert_eq!('1'.in_toggle::<true>(), Ok(Toggle::Enable));
    assert!('2'.in_toggle::<true>().is_err());
    assert!('9'.in_toggle::<true>().is_err());
}

#[test]
fn from_char_relaxed() {
    assert_eq!('0'.in_toggle::<false>(), Ok(Toggle::Disable));
    assert_eq!('1'.in_toggle::<false>(), Ok(Toggle::Enable));
    assert_eq!('2'.in_toggle::<false>(), Ok(Toggle::Enable));
    assert_eq!('9'.in_toggle::<false>(), Ok(Toggle::Enable));
}

#[test]
fn from_str_strict() {
    assert_eq!("0".in_toggle::<true>(), Ok(Toggle::Disable));
    assert_eq!("1".in_toggle::<true>(), Ok(Toggle::Enable));
    assert!("2".in_toggle::<true>().is_err());
    assert!("123456".in_toggle::<true>().is_err());
}

#[test]
fn from_str_relaxed() {
    assert_eq!("0".in_toggle::<false>(), Ok(Toggle::Disable));
    assert_eq!("1".in_toggle::<false>(), Ok(Toggle::Enable));
    assert_eq!("2".in_toggle::<false>(), Ok(Toggle::Enable));
    assert_eq!("123456".in_toggle::<false>(), Ok(Toggle::Enable));
}

#[test]
fn from_word_ok_disable() {
    assert_eq!("disable".in_toggle::<false>(), Ok(Toggle::Disable));
    assert_eq!("false".in_toggle::<false>(), Ok(Toggle::Disable));
    assert_eq!("off".in_toggle::<false>(), Ok(Toggle::Disable));
    assert_eq!("no".in_toggle::<false>(), Ok(Toggle::Disable));

    assert_eq!("DISABLE".in_toggle::<false>(), Ok(Toggle::Disable));
    assert_eq!("FALSE".in_toggle::<false>(), Ok(Toggle::Disable));
    assert_eq!("OFF".in_toggle::<false>(), Ok(Toggle::Disable));
    assert_eq!("NO".in_toggle::<false>(), Ok(Toggle::Disable));

    assert_eq!("DiSaBlE".in_toggle::<false>(), Ok(Toggle::Disable));
    assert_eq!("fAlSe".in_toggle::<false>(), Ok(Toggle::Disable));
    assert_eq!("OfF".in_toggle::<false>(), Ok(Toggle::Disable));
    assert_eq!("nO".in_toggle::<false>(), Ok(Toggle::Disable));
}

#[test]
fn from_word_ok_enable() {
    assert_eq!("enable".in_toggle::<true>(), Ok(Toggle::Enable));
    assert_eq!("true".in_toggle::<true>(), Ok(Toggle::Enable));
    assert_eq!("on".in_toggle::<true>(), Ok(Toggle::Enable));
    assert_eq!("yes".in_toggle::<true>(), Ok(Toggle::Enable));

    assert_eq!("ENABLE".in_toggle::<true>(), Ok(Toggle::Enable));
    assert_eq!("TRUE".in_toggle::<true>(), Ok(Toggle::Enable));
    assert_eq!("ON".in_toggle::<true>(), Ok(Toggle::Enable));
    assert_eq!("YES".in_toggle::<true>(), Ok(Toggle::Enable));

    assert_eq!("EnAbLe".in_toggle::<true>(), Ok(Toggle::Enable));
    assert_eq!("TrUe".in_toggle::<true>(), Ok(Toggle::Enable));
    assert_eq!("On".in_toggle::<true>(), Ok(Toggle::Enable));
    assert_eq!("YeS".in_toggle::<true>(), Ok(Toggle::Enable));
}

#[test]
fn from_word_fail() {
    assert!("".in_toggle::<false>().is_err());
    assert!("not-a-number".in_toggle::<false>().is_err());
    assert!("4294967299".in_toggle::<false>().is_err());
}

#[test]
fn into_u8() {
    assert_eq!(u8::from(Toggle::Disable), 0_u8);
    assert_eq!(u8::from(Toggle::Enable), 1_u8);
}

#[test]
fn into_i8() {
    assert_eq!(i8::from(Toggle::Disable), 0_i8);
    assert_eq!(i8::from(Toggle::Enable), 1_i8);
}

#[test]
fn into_str_default() {
    assert_eq!(<&str>::from(Toggle::Disable), "disable");
    assert_eq!(<&str>::from(Toggle::Enable), "enable");
}

#[test]
fn into_str_diff_kinds() {
    assert_eq!(<&str>::from(Toggle::Disable.display(Kind::Able)), "disable");
    assert_eq!(<&str>::from(Toggle::Enable.display(Kind::Able)), "enable");

    assert_eq!(<&str>::from(Toggle::Disable.display(Kind::Bool)), "false");
    assert_eq!(<&str>::from(Toggle::Enable.display(Kind::Bool)), "true");

    assert_eq!(<&str>::from(Toggle::Disable.display(Kind::Switch)), "off");
    assert_eq!(<&str>::from(Toggle::Enable.display(Kind::Switch)), "on");

    assert_eq!(<&str>::from(Toggle::Disable.display(Kind::Spoken)), "no");
    assert_eq!(<&str>::from(Toggle::Enable.display(Kind::Spoken)), "yes");
}

#[test]
fn matching() {
    for &kind in Kind::VARIANTS {
        let names = kind.names();
        assert_eq!(names.len(), 2, "kind '{}' has {} names", kind, names.len());
        assert_eq!(names[0].in_toggle::<true>(), Ok(Toggle::Disable), "kind '{}' negative name is '{}'", kind, names[0]);
        assert_eq!(names[1].in_toggle::<true>(), Ok(Toggle::Enable), "kind '{}' positive name is '{}'", kind, names[1]);
    }
}