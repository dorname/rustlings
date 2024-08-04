fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        let nice_slice = &a[1..4];

        // 使用 split_at 方法
        // let (_, right) = a.split_at(1); // 跳过前1个元素
        // let (nice_slice, _) = right.split_at(3); // 取接下来的3个元素

        // 此处理方式不对 let nice_slice:Vec<_> = a.iter().skip(1).take(3).collect();
        assert_eq!([2, 3, 4], nice_slice);
    }
}
