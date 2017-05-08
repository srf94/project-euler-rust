use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Clone)]
struct Vertex {
    dist: usize,
    seen: bool,
    defined: bool,
}


impl Vertex {
    fn empty() -> Vertex {
        Vertex {
            dist: 0,
            seen: false,
            defined: false,
        }
    }
}

fn import_file() -> Vec<Vec<usize>> {
    // Create a path to the desired file
    // let path = Path::new("../example.txt");
    let path = Path::new("../question_81_text.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => println!("Read input successfully!"),
    }

    // Parse data into Vector of Vectors (2-D array)
    let line_iterator = input.lines();
    let mut data: Vec<Vec<usize>> = Vec::new();
    for line in line_iterator {
        data.push(line.split(",").map(|s| s.parse::<usize>().unwrap()).collect());
    }
    let square_size = data.len();

    // Need to flip the orientation of the data. [[0, 1], [2, 3]] -> [[0, 2], [1, 3]]
    let mut fliped: Vec<Vec<usize>> = Vec::new();
    for i in 0..square_size {
        let mut inner = Vec::new();
        for vec_1 in &data {
            inner.push(vec_1[i]);
        }
        fliped.push(inner);
    }

    fliped
}

fn main() {
    let data = import_file();
    let square_size = data.len();

    // This vector of vectors will keep track of Dijkstra's progress
    let mut dji = vec![vec![Vertex::empty(); square_size as usize]; square_size as usize];

    dji[0][0] = Vertex{dist: data[0][0], seen: false, defined: true};

    for _ in 0..square_size*square_size {
        // Pick min dist vertex in dji with defined == true, seen == false
        let mut min_dist = -1 as isize;
        let mut new_vertex = (0, 0);
        for (x, vec) in dji.iter().enumerate() {
            for (y, vertex) in vec.iter().enumerate() {
                if !(vertex.defined == true && vertex.seen == false) { continue; }

                if min_dist == -1 || (vertex.dist as isize) < min_dist {
                    min_dist = vertex.dist as isize;
                    new_vertex = (x, y);
                }
            }
        }
        let (x, y) = new_vertex;

        // Remove this vertex from future consideration
        dji[x][y].seen = true;

        // For every neighbour of (x, y) with seen == false
        let mut neighbours = Vec::new();
        if x != 0 { neighbours.push((x-1, y)); }
        if y != 0 { neighbours.push((x, y-1)); }
        if x < square_size-1 { neighbours.push((x+1, y)); }
        if y < square_size-1 { neighbours.push((x, y+1)); }

        for n in neighbours.iter() {
            let (nx, ny) = (n.0, n.1);
            if nx < 0 || ny < 0 { continue; }
            if dji[nx][ny].seen == true { continue; }

            let alt = dji[x][y].dist + data[nx][ny];

            // If alt < current vertex dist, update vertex with new dist
            if dji[nx][ny].defined {
                if alt < dji[x][y].dist {
                    continue;
                }
            }

            dji[nx][ny].dist = alt;
            dji[nx][ny].defined = true;
        }
    }

    println!("Minimum path sum: {:?}", dji[square_size-1][square_size-1].dist);

}




