mod vector_util;

use vector_util::VectorUtil;

use advent::read_stdin;

fn main() {

    let input = read_stdin().join("");

    let mut aux: Vec<char> = Vec::new();

    for (i, item) in input.chars().enumerate() {
        if aux.len() < 4 {
            aux.push(item);
        } else {
            let repeat = aux
                .iter()
                .fold(0, |carry, current| carry + VectorUtil::count(&aux, *current)) ;

            /*
                since VectorUtil::count counts all the occurrences a given item has in a vector,
                its safe to say that if there are no duplicates counts will equal 4, since every
                item apear on the vector at least once,

                therefore if repeat == 4 then there are no duplicates aux array
            */
            if repeat == 4 {
                println!("{}", i);
                return;
            }

            aux.remove(0);
            aux.push(item);
        }
    }

}
