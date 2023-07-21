pub use super::*;

impl Store {


    pub fn euclidean_distance(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
        v1.iter()
            .zip(v2.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt()
    }
    pub fn dot_product(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
        v1.iter().zip(v2.iter()).map(|(x, y)| x * y).sum::<f32>()
    }


    pub fn cosine_similarity(&self, v1: &Vec<f32>, v2: &Vec<f32>) ->  Option<f32>  {
        if v1.len() != v2.len() {
            return None;
        }
        let dot_product = Store::dot_product(v1, v2);
        // 内积开方法计算向量长度
        let magnitude_v1 = Store::dot_product(v1, v1).sqrt();
        let magnitude_v2 = Store::dot_product(v2, v2).sqrt();

        Some(dot_product / (magnitude_v1 * magnitude_v2))
    }

    // AI 自动生成： 一个基于余弦相似度的近邻搜索
    pub fn knn(&self, query: &Vec<f32>, k: usize) -> Vec<(String, &Vec<f32>)> {
        let mut neighbors = self
            .inner
            .iter()
            .map(|(id, vector)| (id.clone(), Store::euclidean_distance(query, vector)))
            .collect::<Vec<_>>();
        neighbors.sort_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap());
        neighbors
            .into_iter()
            .take(k)
            .map(|(id, _)| (id.clone(), self.inner.get(&id).unwrap()))
            .collect()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_dot_product() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![4.0, 5.0, 6.0];
        assert_eq!(super::Store::dot_product(&v1, &v2), 32.0);
    }

    #[test]
    fn test_cosine_similarity() {
        let mut store = super::Store::new();
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![4.0, 5.0, 6.0];
        
        store.insert("1".to_string(), v1.clone());
        store.insert("2".to_string(), v2.clone());
        assert_eq!(store.cosine_similarity(&v1, &v2), Some(0.9746318));
    }


    #[test]
    fn test_knn() {
        let mut store = super::Store::new();
        store.insert("1".to_string(), vec![1.0, 2.0, 3.0]);
        store.insert("2".to_string(), vec![4.0, 5.0, 6.0]);
        store.insert("3".to_string(), vec![7.0, 8.0, 9.0]);
        let neighbors = store.knn(&vec![1.0, 2.0, 3.0], 2);
        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors[0].0, "1");
        assert_eq!(neighbors[1].0, "2");
    }
}