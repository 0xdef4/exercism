pub fn find<C, T>(collection: C, key: T) -> Option<usize>
where
    C: AsRef<[T]>,
    T: PartialEq + PartialOrd + Copy,
{
    if let Err(_) = check_validity_of_search(&collection, key) {
        return None;
    }

    let mut index = find_middle_index(&collection);
    let mut temp;
    while collection.as_ref()[index] != key {
        if key > collection.as_ref()[index] {
            temp = &collection.as_ref()[index + 1..];
            index = index + (find_middle_index(temp) + 1);
        } else {
            temp = &collection.as_ref()[..index];
            index = index - (find_middle_index(temp) + 1);
        }
    }
    Some(index)
}

fn find_middle_index<C, T>(collection: C) -> usize where 
    C: AsRef<[T]>,
 {
    let middle_index;
    if collection.as_ref().len() == 1 {
        middle_index = 0;
    } else if collection.as_ref().len() % 2 == 0 {
        middle_index = collection.as_ref().len() / 2 - 1;
    } else {
        middle_index = collection.as_ref().len() / 2;
    }
    middle_index
}

fn check_validity_of_search<C, T>(collection: C, key: T) -> Result<(), &'static str>
where
    C: AsRef<[T]>,
    T: PartialEq + PartialOrd,
{
    if !collection.as_ref().contains(&key) {
        return Err("collection should contain key");
    } else if collection.as_ref()[0] > key {
        return Err("collection's lowest should be smaller than key");
    } else if collection.as_ref()[collection.as_ref().len() - 1] < key {
        return Err("collection's largest should be bigger than key");
    } else if collection.as_ref().len() == 0 {
        return Err("collection length should not be zero");
    } else {
        Ok(())
    }
}
