use std::cmp::Ordering;
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    if height == 0{
        return vec![];
    }
    let width = minefield[0].len();
    
    
    let mut template = vec![vec![0; width]; height];
    for (i, s) in minefield.into_iter().enumerate() {
        let i = i as i32;
        for (j, c) in s.chars().enumerate() {
            let j = j as i32;
            if c == '*' {
                template[i as usize][j as usize] = -10;
                for e in template[i as usize]
                    [((j - 1).max(0) as usize)..(j as usize + 2).min(width)]
                    .iter_mut()
                {
                    *e += 1;
                }
                if i > 0 {
                    for e in template[i as usize - 1]
                        [((j - 1).max(0) as usize)..(j as usize + 2).min(width)]
                        .iter_mut()
                    {
                        *e += 1;
                    }
                }
                if i as usize + 1 < height {
                    for e in template[i as usize + 1]
                        [((j - 1).max(0) as usize)..(j as usize + 2).min(width)]
                        .iter_mut()
                    {
                        *e += 1;
                    }
                }
            }
        }
    }
    println!("{:?}", template);
    template
        .iter()
        .map(|row| {
            row.iter()
                .map(|count| match (*count).cmp(&0) {
                    Ordering::Equal => " ".to_string(),
                    Ordering::Less => "*".to_string(),
                    _ => format!("{}", *count),
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}
