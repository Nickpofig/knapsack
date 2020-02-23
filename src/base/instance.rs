use super::solution::Solution;
use rand::{Rng};
use std:: 
{
    fmt,
    fmt::Display,
    fmt::Formatter,
    string::*,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Item
{
    pub value: u64,
    pub weight: u64,
}

#[derive(Debug, Clone)]
pub struct Instance
{
    items: Vec<Item>,
    capacity: u64,
}

// properties access
impl Instance
{
    pub fn get_capacity(&self) -> u64 { self.capacity }
    pub fn get_item_count(&self) -> usize { self.items.len() }
}

// methods
impl Instance 
{
    pub fn new () -> InstanceBuilder 
    {
        InstanceBuilder::new()
    } 

    pub fn copy_item_at(&self, index: usize) -> Item 
    {
        self.items[index].clone()
    }

    pub fn solvable_by(&self, solution: &Solution) -> bool 
    {
        let item_count = self.items.len();
        let mut capacity = self.capacity;

        if solution.get_size() != item_count { return false; }

        for i in 0..item_count
        {
            if solution.has_item(i).unwrap()
            {
                capacity -= self.items[i].weight;
            }

            if capacity < self.capacity 
            {
                return false
            }
        }

        true
    }

    pub fn evaluate(&self, solution: &Solution) -> Option<u64> 
    {
        let solution_size = solution.get_size();

        if solution_size != self.get_item_count() 
        {
            return None;
        }

        let mut total_value = 0;
        let mut capacity = self.capacity;
        let mut weight: u64;

        for i in 0..solution_size
        {
            if solution.has_item(i).unwrap()
            {
                total_value += self.items[i].value; 
                weight = self.items[i].weight;

                if capacity >= weight 
                {
                    capacity -= self.items[i].weight;
                }
                else { return None; }
            }
        }

        Some(total_value)
    }
} 

impl PartialEq for Instance 
{
    fn eq(&self, other: &Instance) -> bool 
    {
        if self.capacity != other.capacity { return false; }
        if self.items.len() != other.items.len() { return false; }

        for i in 0..self.items.len() 
        {
            if self.items[i] != other.items[i] { return false; }
        }

        true
    }
}

impl Display for Instance 
{
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result 
    {
        let mut text = String::new();
        
        text += &format!("capacity: {:?}\n", self.capacity);
        
        for i in 0..self.items.len() 
        {
            let value = self.items[i].value;
            let weight = self.items[i].weight;
            
            text += &format! ("[{}] value: {}, weight: {}\n", i, value, weight);
        }

        write! (formatter, "{}", text)
    }
}

#[derive(Debug)]
pub struct InstanceBuilder
{
    instance: Instance,
}

impl InstanceBuilder 
{
    pub fn new() -> Self
    {
        InstanceBuilder 
        {
            instance: Instance
            {
                items: Vec::new(),
                capacity: 0,
            },
        }
    }
    
    pub fn with_capacity(mut self, value: u64) -> Self
    {
        self.instance.capacity = value;
        self
    }

    pub fn add_item(mut self, item: Item) -> Self 
    {
        self.instance.items.push(item);
        self
    }

    pub fn only_add_item(&mut self, item: Item) 
    {
        self.instance.items.push(item);
    }

    pub fn build(self) -> Instance 
    {
        self.instance
    }
}

#[derive(Debug, Copy, Clone)]
pub struct InstanceGenerator 
{
    pub count:      usize,
    pub capacity:   u64,
    pub min_value:  u64,
    pub max_value:  u64,
    pub max_weight: u64,
    pub min_weight: u64,
}

impl InstanceGenerator 
{
    pub fn generate(&self) -> Instance 
    {
        let mut instance = Instance::new().with_capacity(self.capacity);
        let mut random_generator = rand::thread_rng();

        for _ in 0..self.count 
        {
            instance.only_add_item
            (
                Item 
                {
                    value: random_generator.gen_range(self.min_value, self.max_value),
                    weight: random_generator.gen_range(self.min_weight, self.max_weight),
                }
            )
        }

        instance.build()
    }
}

// ** tests **

#[test]
fn test_instance_builder()
{
    let mut instance = Instance::new()
    .with_capacity(1000)
    .add_item
    (
        Item 
        {
            value: 42,
            weight: 21,
        }
    );
    
    instance.only_add_item
    (
        Item 
        {
            value: 666,
            weight: 999,
        }
    );

    let instance = instance.build();

    let right_instance = Instance 
    {
        items: vec! 
        [
            Item 
            {
                value: 42,
                weight: 21,
            },
            Item 
            {
                value: 666,
                weight: 999,
            }
        ],
        capacity: 1000,
    };

    assert_eq!(instance, right_instance);
}