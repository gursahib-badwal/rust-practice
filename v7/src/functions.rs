pub fn next_hailstone(x:u32) -> u32 {
    // unimplemented!();
    if x % 2 == 0 {
        let num:u32 = x/2;
        return num;
    }

    else {
        let num:u32 = 3*x + 1;
        return num;
    }
}

pub fn hailstone_sequence(init:u32) -> Vec<u32> {
    // unimplemented!();
    let mut temp:u32 = init;
    let mut v:Vec<u32> = Vec::new();
    v.push(temp);

    while temp != 1 {
        temp = next_hailstone(temp);
        v.push(temp);
    }
    
    return v;
}

pub fn find_elt<T : Eq>(v: Vec<T>,elt: T) -> Option<usize> {
    // unimplemented!();
    let mut i:usize = 0;
    while i < v.len() {
        if v[i] == elt {
            return Some(i);
        }
        i = i+1;
    }

    return None;
}

pub fn all_indices<T : Eq>(v: Vec<T>,elt: T) -> Vec<usize> {
    // unimplemented!();
    let mut i:usize = 0;
    let mut result_vector:Vec<usize> = Vec::new();

    while i < v.len() {
        if v[i] == elt {
            result_vector.push(i);
        }
        i = i+1;
    }
    return result_vector;
}