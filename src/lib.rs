mod strings;

pub use strings::*;

use logos::Logos;

#[derive(Logos, Debug)]
#[logos(skip r"[ \t\n\f]+")]
enum Partes {
    #[token("de", ignore(case))]
    De,
    #[token("la", ignore(case))]
    La,
    #[token("del", ignore(case))]
    Del,
    #[token("y", ignore(case))]
    Y,
    #[regex("[^ \t\n]{1,}", priority = 0)]
    Parte,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nombre = "John Michael Smith Johnson";
        println!(
            "{:?}",
            Partes::lexer(nombre)
                .map(|e| e.unwrap())
                .collect::<Vec<Partes>>()
        );

        let nombre = "Alice Rodriguez y Garcia";
        println!(
            "{:?}",
            Partes::lexer(nombre)
                .map(|e| e.unwrap())
                .collect::<Vec<Partes>>()
        );

        let nombre = "Sarah Elizabeth Martinez de Cruz";
        println!(
            "{:?}",
            Partes::lexer(nombre)
                .map(|e| e.unwrap())
                .collect::<Vec<Partes>>()
        );

        let nombre = "Sarah Elizabeth Martinez de la Cruz";
        let mut partes_iter = Partes::lexer(nombre);
        while let Some(Ok(parte)) = partes_iter.next() {
            print!("{} ({parte:?}) - ", partes_iter.slice())
        }
    }
}
