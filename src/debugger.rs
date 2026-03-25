use crate::locale::Locale;

pub fn debug_header(verbose: bool, locale: Locale, file: &str) {
    if !verbose {
        return;
    }

    println!("== qsisp ==");
    println!("locale: {:?}", locale);
    println!("file:   {}", file);
}

pub fn debug_block<T>(verbose: bool, title: &str, items: &[T])
where
    T: std::fmt::Debug,
{
    if !verbose {
        return;
    }

    println!("== {title} ==");
    for (idx, item) in items.iter().enumerate() {
        println!("[{idx:04}] {:?}", item);
    }
}

pub fn debug_line(verbose: bool, label: &str, value: impl std::fmt::Display) {
    if !verbose {
        return;
    }

    println!("{label}: {value}");
}