fn main() {
    {
        let mut vector1 = vec![1, 2, 3];
        let mut vector2 = vec![4, 5, 6];
        vector1.append(&mut vector2);
        assert_eq!(vector1, [1, 2, 3, 4, 5, 6]);
        assert_eq!(vector2, []);
    }
    {
        let mut a = vec![1, 2, 3];
        let b = vec![4, 5, 6];

        a.extend(b.clone());
        assert_eq!(a, [1, 2, 3, 4, 5, 6]);
        assert_eq!(b, [4, 5, 6]);
    }

    {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c: Vec<i32> = a.into_iter().chain(b.into_iter()).collect();
        assert_eq!(c, [1, 2, 3, 4, 5, 6]);
    }

    {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];

        let c: Vec<i32> = a.iter().cloned().chain(b.iter().cloned()).collect();
        assert_eq!(c, [1, 2, 3, 4, 5, 6]);
        assert_eq!(a, [1, 2, 3]);
    }

    {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = [a.clone(), b].concat();
        assert_eq!(c, [1, 2, 3, 4, 5, 6]);
        assert_eq!(a, [1, 2, 3]);
    }
}
