use std::fmt;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ColorPattern {
    #[default]
    BackgroundBase,
    BackgroundStrong,
    BackgroundWeak,
    DangerBase,
    DangerStrong,
    DangerWeak,
    PrimaryBase,
    PrimaryStrong,
    PrimaryWeak,
    SecondaryBase,
    SecondaryStrong,
    SecondaryWeak,
    SuccessBase,
    SuccessStrong,
    SuccessWeak,
}

impl ColorPattern {
    pub const ALL: &'static [Self] = &[
        Self::BackgroundBase,
        Self::BackgroundStrong,
        Self::BackgroundWeak,
        Self::DangerBase,
        Self::DangerStrong,
        Self::DangerWeak,
        Self::PrimaryBase,
        Self::PrimaryStrong,
        Self::PrimaryWeak,
        Self::SecondaryBase,
        Self::SecondaryStrong,
        Self::SecondaryWeak,
        Self::SuccessBase,
        Self::SuccessStrong,
        Self::SuccessWeak,
    ];
}

impl fmt::Display for ColorPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackgroundBase => write!(f, "BackgroundBase"),
            Self::BackgroundStrong => write!(f, "BackgroundStrong"),
            Self::BackgroundWeak => write!(f, "BackgroundWeak"),
            Self::DangerBase => write!(f, "DangerBase"),
            Self::DangerStrong => write!(f, "DangerStrong"),
            Self::DangerWeak => write!(f, "DangerWeak"),
            Self::PrimaryBase => write!(f, "PrimaryBase"),
            Self::PrimaryStrong => write!(f, "PrimaryStrong"),
            Self::PrimaryWeak => write!(f, "PrimaryWeak"),
            Self::SecondaryBase => write!(f, "SecondaryBase"),
            Self::SecondaryStrong => write!(f, "SecondaryStrong"),
            Self::SecondaryWeak => write!(f, "SecondaryWeak"),
            Self::SuccessBase => write!(f, "SuccessBase"),
            Self::SuccessStrong => write!(f, "SuccessStrong"),
            Self::SuccessWeak => write!(f, "SuccessWeak"),
        }
    }
}
