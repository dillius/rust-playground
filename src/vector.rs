
pub struct Vector;

impl Vector {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut pos: i32 = 0;
        let mut value = -99999;
        for i in nums.clone().iter() {
            if i != &value {
                nums[pos as usize] = i.clone();
                pos += 1;
                value = *i;
            }
        }
        return pos;
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut index = 0;
        let mut min_buy_price = prices[0];
        let mut max_sell_price = prices[0];
        let mut total_profit = 0;
        while index < prices.len() - 1 {
            while (index < prices.len() - 1) && (prices[index] >= prices[index + 1]) {
                index += 1
            }
            min_buy_price = prices[index];
            while (index < prices.len() - 1) && (prices[index] <= prices[index + 1]) {
                index += 1
            }
            max_sell_price = prices[index];
            total_profit += max_sell_price - min_buy_price;
        }
        return total_profit;
    }

    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        let mut total_profit = 0;
        for index in 1..prices.len() {
            if(prices[index] > prices[index - 1]) {
                total_profit += prices[index] - prices[index - 1];
            }
        }
        return total_profit;
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let target = k as usize % nums.len();
        nums[..].reverse();
        nums[..target].reverse();
        nums[target..].reverse();
    }

    pub fn rotate2(nums: &mut Vec<i32>, k: i32) {
        nums.rotate_right(k as usize);
    }

    pub fn rotate3(nums: &mut Vec<i32>, k: i32) {
        let mut new_nums = vec![];
        for i in 0..nums.len() {
            let target = (i + (k as usize) + 1) % nums.len();
            new_nums.push(nums[target])
        }
        nums.clear();
        nums.append(&mut new_nums);
    }
}
