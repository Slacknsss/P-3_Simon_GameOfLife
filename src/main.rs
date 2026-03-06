use std::thread;
use std::time::Duration;
fn main() {
    let width:usize = 17;
    let height:usize = 17;
    let mut grid: Vec<Vec<bool>> = vec![vec![false; width]; height];
    //#glider 
    // grid[1][2] = true; 
    // grid[2][3] = true;
    // grid[3][1] = true;
    // grid[3][2] = true;
    // grid[3][3] = true;
    
    // blinker
    // grid[8][7] = true;
    // grid[8][8] = true;
    // grid[8][9] = true;
    
    //le toad 
    grid[8][7] = true;
    grid[8][8] = true;
    grid[8][9] = true;
    grid[9][6] = true;
    grid[9][7] = true;
    grid[9][8] = true;
    loop{
        print!("\x1B[2J\x1B[1;1H");
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
    thread::sleep(Duration::from_millis(100));

        grid = next_generation(&grid, width, height);
        let mut alive = false;
        for y in 0..height {
            for x in 0..width {
                if grid[y][x] { alive = true; }
            }
        }
        if !alive { break; }
    

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
            if grid[y][x] && (neighbors == 2 || neighbors == 3) {
                new_grid[y][x] = true;
            }
            if !grid[y][x] && neighbors == 3 {
                new_grid[y][x] = true;
            }
        }
    }
    return new_grid;
}

