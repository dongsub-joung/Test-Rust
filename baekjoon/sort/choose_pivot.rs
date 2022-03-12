fn choose_pivot<T: Ord>(slice: &[T]) -> usize{
    let (mut ismall, imid, ibig)= (0, slice.len()/2, slice.len()-1);
    if slice[ibig] < slice[ismall] {
        std::mem::swap(&mut ibig, &mut ismall);
    }
    if slice[imid] <= slice[ismall]{
        ismall
    }
    else if slice[ibig] <= slice[imid]{
        ibig
    } else {
        imid
    }
}