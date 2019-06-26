use std::collections::HashMap;

// categorize by sorted string
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hash = HashMap::new();
    for s in strs {
        let mut key = s.clone().into_bytes();
        key.sort();
        hash.entry(key).or_insert(vec![]).push(s);
    }
    hash.into_iter().map(|(_, v)| v).collect()
}

const timestamps: [[i32; 2]; 26] = [
    [4798, 0],
    [4763, 578],
    [4659, 1148],
    [4486, 1701],
    [4249, 2230],
    [3949, 2725],
    [3591, 3182],
    [3182, 3591],
    [2725, 3949],
    [2230, 4249],
    [1701, 4486],
    [1148, 4659],
    [578, 4763],
    [0, 4798],
    [-578, 4763],
    [-1148, 4659],
    [-1701, 4486],
    [-2230, 4249],
    [-2725, 3949],
    [-3182, 3591],
    [-3591, 3182],
    [-3949, 2725],
    [-4249, 2230],
    [-4486, 1701],
    [-4659, 1148],
    [-4763, 578],
];

pub fn group_anagrams_fourier(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hash = HashMap::new();
    for s in strs {
        let mut key = [0i32; 2];
        for i in s.as_bytes().iter().map(|b| (b - b'a') as usize) {
            key[0] += timestamps[i][0];
            key[1] += timestamps[i][1];
        }
        hash.entry(key).or_insert(vec![]).push(s);
    }
    hash.into_iter().map(|(_, v)| v).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ];
        let mut output = group_anagrams_fourier(input);
        output.sort();
        let target = vec![
            vec![String::from("bat")],
            vec![
                String::from("eat"),
                String::from("tea"),
                String::from("ate"),
            ],
            vec![String::from("tan"), String::from("nat")],
        ];
        assert_eq!(output, target);
    }
}
