use super::*;

pub trait Approach 
{
    fn apply_to(&self, instance: &Instance) -> Solution;
}

pub struct BruteForce;

impl Approach for BruteForce 
{
    fn apply_to(&self, instance: &Instance) -> Solution 
    {
        let item_count = instance.get_item_count();
        let mut magnitude = 0;
        let mut should_extend: bool;
        
        let mut best_cost = 0;
        let mut best_solution = Solution::empty(item_count);
        
        let mut current_cost: u64;
        let mut current_solution = Solution::empty(item_count);

        // Starts state-space exploration
        loop
        {
            // Calculates cost for current solution when possible
            if let Some(cost) = instance.evaluate(&current_solution) 
            {
                current_cost = cost;

                // Saves current solution as best when cost has maximum value observed
                if best_cost < current_cost   
                {
                    best_cost = current_cost;
                    best_solution = current_solution.clone();
                }
            }
            
            // Marks that there is no unexplored solution with current magnitude
            should_extend = true;
            
            // Moves to next solution
            for i in 0..magnitude 
            {
                if current_solution.has_item(i).unwrap() == false 
                {
                    current_solution.include(i).unwrap();
                    // Marks that there is at least one solution to explore
                    should_extend = false;
                    break
                } 
            }

            // When true starts new solution search 
            if should_extend 
            { 
                // Clear previous choices
                for i in 0..magnitude 
                {
                    current_solution.exclude(i).unwrap();
                }

                // Extends magnitude
                magnitude += 1; 

                if magnitude <= item_count 
                {
                    // Make first choice (to include last unexplored item)
                    current_solution.include(magnitude - 1).unwrap();
                }
                // Breaks search when complete exploration of state-space                  
                else { break }
            }
        }

        best_solution
    }
}