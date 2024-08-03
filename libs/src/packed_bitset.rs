use fixedbitset::FixedBitSet;

/// 複数bitを1つのチャンクにパックして格納するビットセット
/// ビームサーチなどで3値以上の状態をなるべく小さく保持するために使用する
pub struct PackedBitSet {
    data: FixedBitSet,
    chunk_size: usize,
}

impl PackedBitSet {
    /// チャンクサイズとデータ長を指定して初期化
    ///
    /// チャンクサイズは32以下である必要があり、データ長はチャンクサイズの倍数である必要がある
    pub fn new(chunk_size: usize, length: usize) -> Self {
        let data = FixedBitSet::with_capacity(length * chunk_size);
        PackedBitSet { data, chunk_size }
    }

    /// 指定したインデックスに値をセットする
    ///
    /// `value`は`1 << chunk_size`未満である必要がある
    pub fn set(&mut self, index: usize, value: usize) {
        assert!(value < 1 << self.chunk_size);
        for i in 0..self.chunk_size {
            self.data
                .set(index * self.chunk_size + i, (value >> i) & 1 == 1);
        }
    }

    /// 指定したインデックスの値を取得する
    ///
    /// 返り値は`1 << chunk_size`未満である
    pub fn get(&self, index: usize) -> usize {
        let mut value = 0;
        for i in 0..self.chunk_size {
            if self.data[index * self.chunk_size + i] {
                value |= 1 << i;
            }
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_bitset() {
        let mut bitset = PackedBitSet::new(3, 10);
        for i in 0..10 {
            bitset.set(i, i % 8);
        }
        for i in 0..10 {
            assert_eq!(bitset.get(i), i % 8);
        }
    }

    #[test]
    #[should_panic]
    fn test_packed_bitset_panic() {
        let mut bitset = PackedBitSet::new(3, 10);
        bitset.set(0, 8);
    }
}
