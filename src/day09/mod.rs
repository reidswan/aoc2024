use std::iter;

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let disk = parse_input(INPUT);
    let mut raw_disk = to_raw_disk(&disk);
    compact_raw_disk(&mut raw_disk);
    println!("Part 1: {}", check_sum_raw(&raw_disk));

    let mut raw_disk_p2 = to_raw_disk(&disk);
    compact_whole_files(&mut raw_disk_p2);
    println!("Part 2: {}", check_sum_raw(&raw_disk_p2));
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum FileType {
    Free,
    Occupied(usize),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct FileBlock {
    count: usize,
    file_type: FileType,
}

fn to_raw_disk(input: &Vec<FileBlock>) -> Vec<FileType> {
    input
        .iter()
        .map(|it| iter::repeat_n(it.file_type, it.count))
        .flatten()
        .collect()
}

fn parse_input(input: &str) -> Vec<FileBlock> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .filter(|&(_, c)| c > 0)
        .map(|(i, c)| {
            let file_type = if i % 2 == 0 {
                FileType::Occupied(i / 2)
            } else {
                FileType::Free
            };

            FileBlock {
                count: c as usize,
                file_type,
            }
        })
        .collect()
}

fn compact_raw_disk(disk: &mut Vec<FileType>) {
    let mut last_occupied = disk.len() - 1;
    for i in 0..disk.len() {
        while disk[last_occupied] == FileType::Free {
            last_occupied -= 1;
        }

        if i >= last_occupied {
            break;
        }

        if disk[i] == FileType::Free {
            let tmp = disk[i];
            disk[i] = disk[last_occupied];
            disk[last_occupied] = tmp;
        }
    }
}

fn check_sum_raw(disk: &Vec<FileType>) -> usize {
    disk.iter()
        .enumerate()
        .map(|(i, typ)| match typ {
            FileType::Free => 0,
            FileType::Occupied(id) => id * i,
        })
        .sum::<usize>()
}

fn compact_whole_files(disk: &mut Vec<FileType>) {
    let mut ix = disk.len() - 1;
    loop {
        let chunk_end = ix;
        let chunk_start = find_chunk_start(&disk, chunk_end);

        if disk[chunk_end] != FileType::Free {
            let mut trg_start = 0;
            while trg_start < chunk_start {
                let trg_end = find_chunk_end(disk, trg_start);

                if disk[trg_start] == FileType::Free
                    && (trg_end - trg_start) >= (chunk_end - chunk_start)
                {
                    swap_chunk(disk, chunk_start, trg_start, chunk_end - chunk_start + 1);
                    break;
                } else {
                    trg_start = trg_end + 1;
                }
            }
        }

        if chunk_start == 0 {
            break;
        } else {
            ix = chunk_start - 1;
        }
    }
}

fn swap_chunk(arr: &mut Vec<FileType>, src_start: usize, target_start: usize, len: usize) {
    for ix in 0..len {
        let tmp = arr[target_start + ix];
        arr[target_start + ix] = arr[src_start + ix];
        arr[src_start + ix] = tmp
    }
}

fn find_chunk_start(disk: &Vec<FileType>, end: usize) -> usize {
    let mut start = end;
    while start >= 1 && disk[start - 1] == disk[end] {
        start -= 1;
    }

    start
}

fn find_chunk_end(disk: &Vec<FileType>, start: usize) -> usize {
    let mut end = start;
    while end < disk.len() - 1 && disk[end + 1] == disk[start] {
        end += 1;
    }

    end
}
