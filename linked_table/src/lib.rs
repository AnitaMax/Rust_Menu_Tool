mod linked_table;
pub use linked_table::*;


#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_push() {
        let mut table = LinkedTable::new();
        table.push(1);
        assert_eq!(table.len, 1);
    }

    #[test]
    fn test_find() {
        let mut table = LinkedTable::new();
        table.push(1);
        table.push(2);
        table.push(3);
        assert_eq!(table.find(|x| *x == 2).unwrap().get_data(), &2);
    }

    #[test]
    fn test_for_all() {
        let mut table = LinkedTable::new();
        table.push(1);
        table.push(2);
        table.push(3);
        let mut sum = 0;
        table.for_all(|x| sum += x);
        assert!(sum == 6);
    }

    #[test]
    fn test_pop(){
        let mut table = LinkedTable::new();
        table.push(1);
        table.push(2);
        table.push(3);
        assert_eq!(table.pop().unwrap().get_data(), &3);
        assert_eq!(table.pop().unwrap().get_data(), &2);
        assert_eq!(table.pop().unwrap().get_data(), &1);
        assert!(table.pop().is_none());
    }

    #[test]
    fn test_get_len(){
        let mut table = LinkedTable::new();
        table.push(1);
        table.push(2);
        table.push(3);
        assert_eq!(table.get_len(), 3);
        table.pop();
        assert_eq!(table.get_len(), 2);
    }
}
    