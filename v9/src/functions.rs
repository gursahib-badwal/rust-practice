pub fn prefixes(s:&str) -> Vec<&str> {
    // unimplemented!()
    let mut acc: Vec<&str> = Vec::new();
    let strlen = s.len();

    for i in 0..(strlen+1){
        acc.push(&s[0..i]);
    }

    return acc;
}

pub fn return_if_satisfies_both<'a,'b,T,F:Fn(&T)->bool>(f:F,x:&'a T,y:&'b T) 
-> Option<(&'a T,&'b T)> {
    // unimplemented!()
    if f(x) && f(y) {
        return Some((x,y))
    }
    return None
}

#[derive(Clone,PartialEq,Debug)]
pub enum List<T> {
    Nil,
    Cons(T,Box<List<T>>)
}

pub fn map<T,U,F:Fn(&T)->U>(f:F,l:& List<T>) -> List<U> {
    // unimplemented!()
    match l {
        List::Cons(value, rest) => List::Cons(f(value), Box::new(map(f, rest))),
        List::Nil => List::Nil
    }
}

pub fn concat<T:Copy>(l1:& List<T>,l2:& List<T>) -> List<T> {
    // unimplemented!()
    if let List::Nil = l1 { //Base case for the recursive call
        return l2.clone();
    }

    else if let List::Cons(value, rest) = l1{
        return List::Cons(*value, Box::new(concat(rest, l2)));
    }
 
    else {
        return List::Nil;
    }
}
