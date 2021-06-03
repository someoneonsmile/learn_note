struct AvgVec {
    list: Vec<i32>,
    avg: f64,
}

impl AvgVec {
    /// 添加
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
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

pub fn main() {
    let mut avg_vec = AvgVec {
        list: vec![],
        avg: 0f64,
    };
    avg_vec.add(5);
    avg_vec.add(6);
    avg_vec.add(6);
    avg_vec.remove();
    avg_vec.add(6);
    avg_vec.add(6);
    avg_vec.add(9);
    avg_vec.add(10);
    avg_vec.add(16);
    println!("avg: {}", avg_vec.avg());
}
