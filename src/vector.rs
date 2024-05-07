
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

}
