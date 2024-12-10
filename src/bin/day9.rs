mod util;
#[derive(Debug)]
enum Block {
    Empty,
    Data(u32),
}

#[derive(Debug)]
struct Disk {
    data: Vec<Block>,
    file_count: u32,
}
impl Disk {
    fn parse(line: &String) -> Self {
        let mut output: Vec<Block> = Vec::new();

        let mut file_id: u32 = 0;
        let mut either = true;
        for c in line.chars() {
            let size: usize = c.to_string().parse().expect("should have been a number");
            if either {
                // TODO faster way to add 'size' copies?
                for _x in 0..size {
                    output.push(Block::Data(file_id));
                }
                file_id += 1;
            } else {
                for _x in 0..size {
                    output.push(Block::Empty);
                }
            }
            either = !either;
        }

        Self {
            data: output,
            file_count: file_id,
        }
    }
    fn get_contiguous_free(&self, size: usize) -> Option<usize> {
        // search from the left, see if there are any this size
        let mut start_of_empty: usize = 0;

        for i in 0..self.data.len() {
            match self.data[i] {
                Block::Empty => {
                    if i - start_of_empty >= size {
                        return Some(start_of_empty + 1);
                    }
                }
                Block::Data(_) => start_of_empty = i,
            }
        }

        None
    }
    fn checksum(&self) -> u64 {
        let mut total = 0;

        for i in 0..self.data.len() {
            match self.data[i] {
                Block::Empty => {}
                Block::Data(file_id) => {
                    total += file_id as u64 * i as u64;
                }
            }
        }

        total
    }
    fn index_of_last_block(&self) -> usize {
        for i in (0..self.data.len()).rev() {
            match self.data[i] {
                Block::Empty => {}
                Block::Data(file_id) => return i,
            }
        }
        panic!("No data???");
    }
    fn print(&self) {
        for i in 0..self.data.len() {
            match self.data[i] {
                Block::Empty => print!("."),
                Block::Data(file_id) => print!("{file_id}"),
            }
        }
        println!("");
    }
}

fn main() {
    println!("AoC 2024: Day 9");

    // let lines = util::get_lines_from_file("input/day9-test.txt");
    let lines = util::get_lines_from_file("input/day9.txt");

    let input = &lines[0];

    fn part1(input: &String) {
        let mut disk = Disk::parse(input);

        println!("Disk loaded, sorting data...");
        // disk.print();

        // look for empty blocks from the left
        for i in 0..disk.data.len() {
            match disk.data[i] {
                Block::Empty => {
                    let last_block_idx = disk.index_of_last_block();
                    if last_block_idx <= i {
                        // if all the data is already to the left
                        // of the current position, we're done
                        break;
                    }
                    disk.data.swap(i, last_block_idx);
                }
                Block::Data(_) => {}
            }
        }
        println!("Data sorted, calculating checksum...");

        // disk.print();
        println!("Checksum: {}", disk.checksum());
    }

    // let mut disk = Disk::parse(input);
    // // disk.print();
    // // TODO: record the maximum file_id in a new field in the Disk struct
    // // loop over all files starting with the max (in reverse):
    // //      determine the length of the file
    // //      search for free space of that size
    // //      if it exists:
    // //          if it's lower than the current location of the file,
    // //          move the file (move each location)
    // //          if it's not lower, continue
    // //      if it doesn't exist:
    // //          continue
    // dbg!(
    //     disk.get_contiguous_free(/* size */ 4)
    // );

    // for i in (0..disk.file_count).rev() {
    //     // println!("{}", i);
    //     let indices: Vec<_> = disk.data
    //         .iter()
    //         .enumerate()
    //         .filter_map(|(ii, foo)| {
    //             match foo {
    //                 Block::Data(d) => if *d == i { Some(ii) } else { None },
    //                 _ => None
    //             }
    //         })
    //         .collect();

    //     if let Some(free_idx) = disk.get_contiguous_free(indices.len()) {
    //         if free_idx >= indices[0] {
    //             continue;
    //         }
    //         // dbg!(indices);
    //         // dbg!(idx);
    //         // move it
    //         // println!("file_id {i} can fit at {free_idx}", );
    //         let mut start_free_idx = free_idx;
    //         for idx in indices {
    //             disk.data.swap(start_free_idx, idx);
    //             start_free_idx += 1;
    //         }
    //         // disk.print();
    //     } else {
    //         // println!("This one is too big! {i}");
    //     }
    // }

    // disk.print();
    // dbg!(disk.checksum());

    part1(&input.clone());
}
