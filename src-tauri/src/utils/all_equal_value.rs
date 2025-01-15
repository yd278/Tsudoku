pub trait AllEqualValue: Iterator {
    fn all_equal_value(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: PartialEq,
    {
        let mut iter = self;
        let first = iter.next(); // 获取第一个元素

        match first {
            None => return None, // 没有元素，返回 None
            Some(first_value) => {
                // 检查后续元素是否与第一个元素相等
                if iter.all(|x| x == first_value) {
                    return Some(first_value); // 返回第一个元素的 Some 值
                }
            }
        }

        None // 如果不相等，返回 None
    }
}

impl<T: ?Sized> AllEqualValue for T where T: Iterator {}
