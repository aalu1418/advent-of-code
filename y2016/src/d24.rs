use itertools::Itertools;

pub fn twentyfour(input: Vec<String>) -> (String, String) {
    let mut nodes: Vec<(usize, usize)> = vec![];
    let mut start: (usize, usize) = (0, 0);
    let map: Vec<Vec<char>> = input[0]
        .split('\n')
        .enumerate()
        .map(|(i, r)| {
            return r
                .chars()
                .enumerate()
                .map(|(j, v)| {
                    if v == '0' {
                        start = (i, j);
                    }

                    if v != '#' && v != '.' && v != '0' {
                        nodes.push((i, j));
                    }

                    return v;
                })
                .collect::<Vec<char>>();
        })
        .collect();

    nodes.insert(0, start);
    let mut dist: Vec<Vec<usize>> = vec![vec![0; nodes.len()]; nodes.len()];
    for x in 0..nodes.len() {
        for y in 0..nodes.len() {
            if x == y || dist[x][y] != 0 {
                continue;
            }
            let d = shortest_path(map.clone(), nodes[x], nodes[y]);
            dist[x][y] = d;
            dist[y][x] = d;
        }
    }

    // generate permutations
    let inds: Vec<usize> = (1..nodes.len()).collect();
    let mut min_path = input[0].len();
    for mut p in inds.iter().permutations(nodes.len() - 1).unique() {
        p.insert(0, &0);

        let mut sum: usize = 0;
        for i in 0..p.len() - 1 {
            let from: usize = *p[i];
            let to: usize = *p[i + 1];
            sum += dist[from][to];
        }

        if sum < min_path {
            min_path = sum;
        }
    }
    let mut min_path_return = input[0].len();
    for mut p in inds.iter().permutations(nodes.len() - 1).unique() {
        p.insert(0, &0);
        p.push(&0);

        let mut sum: usize = 0;
        for i in 0..p.len() - 1 {
            let from: usize = *p[i];
            let to: usize = *p[i + 1];
            sum += dist[from][to];
        }

        if sum < min_path_return {
            min_path_return = sum;
        }
    }

    (min_path.to_string(), min_path_return.to_string())
}

#[derive(Clone, Debug)]
struct Step {
    current: (usize, usize),
    goal: (usize, usize),
    steps: usize,
}

impl Step {
    fn next(&mut self, visited: &mut Vec<Vec<char>>) -> Vec<Step> {
        return vec![
            (self.current.0, self.current.1 - 1),
            (self.current.0, self.current.1 + 1),
            (self.current.0 - 1, self.current.1),
            (self.current.0 + 1, self.current.1),
        ]
        .iter()
        .filter_map(|x| {
            if visited[x.0][x.1] != '#' && visited[x.0][x.1] != 'X' {
                let mut s = self.clone();
                visited[x.0][x.1] = 'X';
                s.current = *x;
                s.steps += 1;
                return Some(s);
            }
            return None;
        })
        .collect();
    }

    fn done(&self) -> bool {
        self.goal == self.current
    }
}

fn shortest_path(map: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut map = map.clone();
    map[start.0][start.1] = 'X';
    let mut list: Vec<Step> = vec![Step {
        current: start,
        goal: end,
        steps: 0,
    }];

    return find_path(&mut list, &mut map);
}

fn find_path(s: &mut Vec<Step>, visited: &mut Vec<Vec<char>>) -> usize {
    let mut x = s[0].clone();
    s.remove(0);

    if x.done() {
        return x.steps;
    }

    let mut n = x.next(visited);
    if n.len() != 0 {
        s.append(&mut n);
    }

    return find_path(s, visited);
}
