#[derive(Debug)]
pub struct Solution 
{
    items: Vec<u8>
}

impl Solution 
{
    pub fn random(item_count: usize) -> Self
    {
        let mut solution = Solution 
        {
            items: Vec::with_capacity(item_count)
        };

        for _ in 0..item_count
        {
            let flag = if rand::random() {1} else {0};
            solution.items.push(flag);
        }

        solution
    }

    pub fn empty(item_count: usize) -> Solution 
    { 
        Solution 
        {
            items: vec! [0; item_count],
        }
    }

    pub fn has_item(&self, index: usize) -> Result<bool, String>
    {
        if index >= self.get_size() { return Self::get_out_of_range_error(); }

        Ok(self.items[index] == 1)
    }

    pub fn include(&mut self, index: usize) -> Result<(), String>
    {
        if index >= self.get_size() { return Self::get_out_of_range_error(); }

        self.items[index] = 1;
        Ok(())
    }

    pub fn exclude(&mut self, index: usize) -> Result<(), String>
    {
        if index >= self.get_size() { return Self::get_out_of_range_error(); }

        self.items[index] = 0;
        Ok(())
    }

    pub fn get_size(&self) -> usize 
    {
        self.items.len()
    }

    fn get_out_of_range_error<T>() -> Result<T, String> 
    {
        Err("index is out of range".to_string())
    } 
}

impl Clone for Solution 
{
    fn clone(&self) -> Self
    {
        Solution 
        {
            items: self.items.clone()
        }
    }
}