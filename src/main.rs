use std::str::Chars;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {

    let lines = lines_from_file("keypadCode.txt");
    //for current_line in lines {
        //println!("{:?}", current_line);
       // process_line(current_line,0);

    //}

    lines.into_iter().fold(0i16, |x, line| process_line(line,x));

    //let sum = v.into_iter().fold(acc, |xy, x| lets_walk(xy.p,xy.o,build_route(x)));

}


fn lines_from_file<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}



pub fn process_line(current_line: String, current_position: i16) -> i16 {

    println!("{:?}", current_line);

    let finish_position = current_line.chars().fold(current_position, |x, next_character| calculate_next(x,next_character));
    println!("{:?}", finish_position);
    finish_position

}


pub fn calculate_next(x: i16, movement: char) -> i16 {

    match movement {

        'U' => if x < 4 {x} else {x -3},
        'D' => if x > 6 {x} else {x + 3},
        'R' => if (x % 3)==0 {x} else {x +1},
        'L' => if (x+2)%3 == 0 {x} else {x-1},
        _ => panic!("Invalid character should be one of 'U,D,R,L: {}", x),
    }

}
