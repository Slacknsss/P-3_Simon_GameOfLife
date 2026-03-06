// fn main() {

//     let width = 20;
//     let height = 10;

//     let grid: Vec<Vec<bool>> = vec![vec![false; width]; height];

//     for row in grid {
//         for cell in row {
//             if cell {
//                 print!("⬛");
//             } else {
//                 print!("⬜");
//             }
//         }
//         println!();
//     }
// }
// fn main() {
//     for _y in 0..5{
//         for _x in 0..5{
//             print!(".");
//         }
//         println!();
//     }
// }

fn main() {

    let width = 5;
    let height = 5;

    for y in 0..height {

        for x in 0..width {

            if x == 2 && y == 2 {
                print!("⬛");
            } else {
                print!("⬜");
            }

        }

        println!();

    }

}

