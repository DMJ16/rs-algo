pub fn topological_sort(jobs: i32, deps: Vec<Vec<i32>>) -> Vec<i32> {
    let mut v: Vec<(usize, Vec<usize>)> = vec![(0, vec![]); jobs as usize];
    for p in deps.iter() {
        v[p[0] as usize].0 += 1;
        v[p[1] as usize].1.push(p[0] as usize);
    }
    let mut stack: Vec<usize> = Vec::new();
    for (i, e) in (0..).zip(v.iter()) {
        if e.0 == 0 {
            stack.push(i);
        }
    }
    let mut answer: Vec<i32> = Vec::with_capacity(jobs as usize);
    while let Some(last) = stack.pop() {
        answer.push(last as i32);
        for i in v[last].1.clone() {
            v[i].0 -= 1;
            if v[i].0 == 0 {
                stack.push(i);
            }
        }
    }
    if answer.len() == jobs as usize {
        answer
    } else {
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn test_topological() {
        assert_eq!(
            topological_sort(
                4,
                vec![vec![1, 2], vec![1, 3], vec![3, 2], vec![4, 2], vec![4, 3]]
            ),
            [1, 4, 3, 2]
        );
        assert_eq!(
            topological_sort(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            vec![0, 2, 1, 3]
        );
        assert_eq!(topological_sort(2, vec![vec![1, 0]]), vec![0, 1]);
    }
}
