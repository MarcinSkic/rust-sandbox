fn main() {
    // manual calling of iterator
    {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    // lazy iterators
    {
        let v1: Vec<i32> = vec![1, 2, 3];

        // creating iter and calling map is lazy, values are calculated on call of collect()
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    // subtle difference with references in filter and map
    {
        let v = vec![1, 2, 3, 4];
        let a: Vec<_> = v
            .iter()
            .filter(|x: &&i32| *x % 2 == 0)
            .map(|x: &i32| x * 2)
            .collect();
        let b: Vec<_> = v
            .iter()
            .map(|x: &i32| x * 2)
            .filter(|x: &i32| x % 2 == 0)
            .collect();
        println!("{} {}", a[0], b[0]);
    }
}
