/*
 * usage:
 * cargo run --bin ignored
 */

fn main() {
    let filename = "generated/dates_and_paths.txt";

    let ignore = [
        "book/src/SUMMARY",
        "home/src/pages/communidad",   /* communidad.rs */
        "home/src/pages/contributors", /* contributors.rs */
        "home/src/pages/mod",          /* mod.rs */
        /* dotnet*/
        "dotnet/src/es/SUMMARY",
        "dotnet/src/es/license",
    ];
    let content = std::fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Failed to open '{filename}' file"));

    let items: Vec<_> = content
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            // * if err over here hence format err, check your commas❗
            (parts[0], parts[1])
        })
        .filter(|(_date, path)| {
            ignore.contains(&path)
        })
        .collect();

    println!("{:#?}", items);
}
