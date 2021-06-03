/// Example
///
/// ```
/// # fn main() {
/// use avg::AvgVec;
///
/// let mut avg_vec = AvgVec::new();
/// avg_vec.add(6);
/// avg_vec.add(6);
/// avg_vec.add(6);
/// avg_vec.remove();
/// println!("avg: {}", avg_vec.avg());
/// assert_eq!(avg_vec.avg(), 6.0);
/// # }
/// ```
#[derive(Debug)]
pub struct AvgVec {
    list: Vec<i32>,
    avg: f64,
}

impl AvgVec {
    pub fn new() -> AvgVec {
        AvgVec {
            list: vec![],
            avg: 0_f64,
        }
    }

    /// 添加
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_avg();
    }

    /// 删除值
    pub fn remove(&mut self) -> Option<i32> {
        let ret = self.list.pop();
        match ret {
            Some(value) => {
                self.update_avg();
                Some(value)
            }
            None => None,
        }
    }

    /// 获取平均值
    pub fn avg(&self) -> f64 {
        self.avg
    }

    /// 更新平均值
    fn update_avg(&mut self) {
        // 方法泛型 `::<>()`
        self.avg = self.list.iter().sum::<i32>() as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut avg_vec = AvgVec {
            list: vec![],
            avg: 0_f64,
        };
        avg_vec.add(6);
        avg_vec.add(6);
        avg_vec.add(6);
        avg_vec.remove();
        println!("avg: {}", avg_vec.avg());
        assert_eq!(avg_vec.avg(), 6.0);
    }
}
