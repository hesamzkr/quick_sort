pub fn quick_sort(vector: Vec<i32>) -> Vec<i32> {
    if vector.len() > 1 {
        let pivot = vector[vector.len() / 2 as usize];
        let mut less: Vec<i32> = Vec::new();
        let mut equal: Vec<i32> = Vec::new();
        let mut greater: Vec<i32> = Vec::new();
        for elem in vector {
            if elem < pivot {
                less.push(elem);
            } else if elem == pivot {
                equal.push(elem);
            } else {
                greater.push(elem);
            }
        }

        vector_merge(vec![quick_sort(less), equal, quick_sort(greater)])
    } else {
        vector
    }
}

fn vector_merge(vectors: Vec<Vec<i32>>) -> Vec<i32> {
    let mut resultant = Vec::new();

    for vector in vectors {
        for elem in vector {
            resultant.push(elem);
        }
    }

    resultant
}
