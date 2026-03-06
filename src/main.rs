use std::thread;
use std::time::Duration;

fn main() {
    // Taille de la grille
    let width: usize = 17;
    let height: usize = 17;

    // Création de la grille : toutes les cellules mortes au départ
    let mut grid: Vec<Vec<bool>> = vec![vec![false; width]; height];

    // Pattern de départ : le Toad (oscillateur 2 ticks)
    grid[8][7] = true;
    grid[8][8] = true;
    grid[8][9] = true;
    grid[9][6] = true;
    grid[9][7] = true;
    grid[9][8] = true;

    // Boucle principale du jeu
    loop {
        // Efface le terminal avant chaque affichage
        print!("\x1B[2J\x1B[1;1H");

        // Affiche la grille ligne par ligne
        for y in 0..height {
            for x in 0..width {
                if grid[y][x] {
                    print!("⬛"); // Cellule vivante
                } else {
                    print!("⬜"); // Cellule morte
                }
            }
            println!();
        }

        // Pause entre chaque génération
        thread::sleep(Duration::from_millis(100));

        // Calcule et remplace la grille par la génération suivante
        grid = next_generation(&grid, width, height);

        // Arrête le jeu si plus aucune cellule n'est vivante
        let mut alive = false;
        for y in 0..height {
            for x in 0..width {
                if grid[y][x] { alive = true; }
            }
        }
        if !alive { break; }
    }
}

// Compte le nombre de voisins vivants autour de la cellule (x, y)
fn count_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize, width: usize, height: usize) -> u8 {
    let mut count = 0;

    // Les 8 directions possibles autour d'une cellule
    let voisin: [(i32, i32); 8] = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    for (dx, dy) in voisin {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        // Vérifie que le voisin est bien dans la grille avant d'y accéder
        if ny >= 0 && nx >= 0 && ny < height as i32 && nx < width as i32 {
            if grid[ny as usize][nx as usize] {
                count += 1;
            }
        }
    }
    return count;
}

// Calcule l'état de la grille au prochain tick selon les règles du Jeu de la Vie
fn next_generation(grid: &Vec<Vec<bool>>, width: usize, height: usize) -> Vec<Vec<bool>> {
    // Nouvelle grille vide (toutes les cellules mortes par défaut)
    let mut new_grid = vec![vec![false; width]; height];

    for y in 0..height {
        for x in 0..width {
            let neighbors = count_neighbors(grid, x, y, width, height);

            // Une cellule vivante survit avec 2 ou 3 voisins
            if grid[y][x] && (neighbors == 2 || neighbors == 3) {
                new_grid[y][x] = true;
            }
            // Une cellule morte devient vivante avec exactement 3 voisins
            if !grid[y][x] && neighbors == 3 {
                new_grid[y][x] = true;
            }
        }
    }
    return new_grid;
}