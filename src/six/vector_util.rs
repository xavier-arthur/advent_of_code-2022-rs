pub trait VectorUtil {
    type Item;

    fn count(&self, elem: Self::Item) -> usize;
}

impl <T>VectorUtil for Vec<T>
where
    T: PartialEq
{

    type Item = T;

    fn count(&self, elem: Self::Item) -> usize
    {
        let mut total = 0usize;

        for item in self {
            if *item == elem {
                total += 1;
            }
        }

        total
    }
}
