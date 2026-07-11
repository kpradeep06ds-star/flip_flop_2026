#[derive(Clone, Debug)]
// I don't want to mutate so instead what am I doing is this
// I am marking if the leaf is broken
// to do this I should put the entire structure into a struct otherwise I have to run two parallel variables (which is highly inefficient)
pub struct Leaf {
    side: String,
    is_broken: bool,
}

fn simulate_one_worker(leaves: &mut Vec<Leaf>) -> bool {

    // I am trying to figure out the first branch
    // the leaves carries only the LL and LR rest of them are filtered
    // I don't need to track the flower as such I just need to know if it's the first leave from top
    // I also have reversed value due the problem of storage of accumulating them from top to bottom , so bottom would be first hence the reverse

    // it returns the current index which can be starting point
    // the moment there is no starting point we need to stop the iteration
    // this should be used in while loop as stopping condition
    // Also always note leaves is a struct so it will have boolean and a vector embded
    let  current_idx = match leaves.iter().position(|l| !l.is_broken) {
        Some(idx) => idx,
        None => return false, // No leaves left at all!
    };
    // once we are into a safe place to start
    // the last_processed_idx is current
    let mut last_processed_idx = current_idx;
    // let mut walked_at_least_once = false;
    // start point is never swapped ,so we start with  +1 count of current index

    // Move upwards through the remaining leaves
    for i in (current_idx + 1)..leaves.len() {
        if leaves[i].is_broken {
            continue; // Skip broken leaves
        }
        // Ignore wherever we have issues with leaves which are borken

        // walked_at_least_once = true;

        // If the side changes, the worker swaps sides
        if leaves[i].side != leaves[last_processed_idx].side {
            // Rule: The leaf they pushed off from snaps off
            leaves[last_processed_idx].is_broken = true;
        }
        // Imagine a case where nothing remains for swap, in that case las_processed_idx of the loop will be broken
        // this case should be done immediately after the loop
        // Move worker to the new leaf
        last_processed_idx = i;
    }

    // The final leap to the flower head breaks the last leaf they stood on.
    // If they never moved from the first leaf, that first leaf is the last leaf.
    leaves[last_processed_idx].is_broken = true;

    true
}

pub fn count_successful_workers(text: String) -> i32 {
    // Parse out initial leaves from bottom to top
    let mut leaves: Vec<Leaf> = text
        .lines()
        .map(|line| line.trim())
        .filter(|&line| line == "o-|" || line == "|-o")
        .map(|line| Leaf {
            side: if line == "o-|" { "LL".to_string() } else { "LR".to_string() },
            is_broken: false,
        })
        .collect();
    
    // Reverse because we climb from bottom to top
    leaves.reverse();

    let mut total_workers = 0;

    // Simulating workers one by one
    while simulate_one_worker(&mut leaves) {
        total_workers += 1;
    }

    total_workers
}