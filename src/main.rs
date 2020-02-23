#![allow(dead_code)]

mod approach;
use approach::*;

mod base;
use base:: 
{
    solution::*,
    instance::*,
};

use console::Term;

fn main()
{
    let terminal = Term::stdout();
    
    terminal.set_title("knapsack");

    let instance_generator = InstanceGenerator 
    {
        count: 4,
        capacity: 500,
        min_value: 100,
        max_value: 1000,
        min_weight: 20,
        max_weight: 500,
    };

    let instance = instance_generator.generate();

    terminal.write_line(&format!("{}", &instance)).ok();

    let brute_force = approach::BruteForce;
    let solution = brute_force.apply_to(&instance);

    terminal.write_line(&format!("{:?}", &solution)).ok();
}
