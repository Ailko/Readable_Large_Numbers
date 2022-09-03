fn main() {
    const prefix_over_9 = (
        ("un", "duo", "tre", "quattuor", "quinqua", "se", "septe", "octo", "nove"),
        ("deci", "viginti", "triginta", "quadraginta", "quinquaginta", "sexaginta", "septuaginta", "octoginta", "nonaginta"),
        ("cent", "ducent", "trecent", "quadringent", "quingent", "sescent", "septingenti", "octingent", "nongent"),
    );
    const prefix_under_10 = ("m", "b", "tr", "quadr", "quint", "sext", "sept", "oct", "non");
    const modifiers = (
        (
            ("", "n"),
            ("s", "m"),
            ("s", "n"),
            ("s", "n"),
            ("s", "n"),
            ("", "n"),
            ("", "n"),
            ("x", "m"),
            ("", ""),
        ),
        (
            ("x", "n"),
            ("", "n"),
            ("s", "n"),
            ("s", "n"),
            ("s", "n"),
            ("", "n"),
            ("", "n"),
            ("x", "m"),
            ("", ""),
        ),
    );
}

