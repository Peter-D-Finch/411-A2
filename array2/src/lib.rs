// TRAITS AND STRUCTS
pub trait Array2 {
    type Item;
    fn get(&self, x: i32, y: i32) -> Self::Item;
    fn iter_row_major(&self) -> ArrIter<Self::Item>;
    fn iter_col_major(&self) -> ArrIter<Self::Item>;
    fn from_row_major(vector: Vec<Self::Item>, row_len: i32) -> Self;
}
pub struct Arr<T> {
    pub data: Vec<T>,
    pub width: i32,
    pub height: i32
}
pub struct ArrIter<'a, T> {
    pub index: i32,
    pub src: &'a Arr<T>,
    pub cmajor: bool
}

// IMPLEMENTATIONS
impl Iterator for ArrIter<'_, i32> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cmajor == true {
            if self.index < self.src.data.len() as i32 {
                let y_pos = self.index % self.src.width;
                let x_pos = (self.index - y_pos)/self.src.width;
                let result = self.src.get(y_pos, x_pos);
                self.index = self.index + 1;
                Some(result)
            }
            else { None }
            
        }
        else if self.cmajor == false {
            if self.index < self.src.data.len() as i32 {
                let y_pos = self.index % self.src.width;
                let x_pos = (self.index - y_pos)/self.src.width;
                let result = self.src.get(x_pos, y_pos);
                self.index = self.index + 1;
                Some(result)
            }
            else { None }
        }
        else { None }
    }
}
impl Array2 for Arr<i32> {
    type Item = i32;
    fn get(&self, x: i32, y: i32) -> i32 {
        let index = x*self.width + y;
        return self.data[index as usize];
    }
    fn iter_row_major(&self) -> ArrIter<i32> {
        let iter = ArrIter {index: 0, src: self, cmajor: false};
        return iter;
    }
    fn iter_col_major(&self) -> ArrIter<i32> {
        let iter = ArrIter {index: 0, src: self, cmajor: true};
        return iter;
    }
    fn from_row_major(vector: Vec<i32>, row_len: i32) -> Self {
        let length = vector.len();
        Arr {
            data: vector,
            width: row_len,
            height: (length as i32)/row_len
        }
    }
}