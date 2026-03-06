fn main() {
    let width:usize = 17;
    let height:usize = 17;
    let mut grid: Vec<Vec<bool>> = vec![vec![false; width]; height];
    grid[4][4] = true;
    for y in 0..height {

        for x in 0..width {
            if grid[y][x] {
                print!("⬛");
            } else {
                print!("⬜");
            }

        }

        println!();

    }

}



fn count_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize, width: usize ,height: usize ) -> u8 {
    let mut count = 0;
    let voisin: [(i32, i32); 8] = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),           (1, 0),
        (-1, 1),  (0, 1),  (1, 1),        
    ];
    for (dx, dy) in voisin{
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if ny >= 0 && nx >= 0 && ny < height as i32 && nx < width as i32 {
            if grid[ny as usize][nx as usize]{
                count += 1;}
        }

    }
    return count;
}

fn next_generation(grid: &Vec<Vec<bool>>, width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut new_grid = vec![vec![false; width]; height];
    for y in 0..height {
        for x in 0..width {
            let neighbors = count_neighbors(grid, x, y, width, height);
            
        }
    }
    return new_grid;
}

