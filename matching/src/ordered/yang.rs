use crate::Matchings;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Direction {
    Top,
    Left,
    Diag,
}

#[derive(Clone)]
struct Entry<'a>(pub Direction, pub Matchings<'a>);

impl Default for Entry<'_> {
    fn default() -> Self {
        Self(Direction::Top, Default::default())
    }
}

// Returns the maximum matching between the children
pub fn yang<'a>(
    left_children: &'a [model::CSTNode],
    right_children: &'a [model::CSTNode],
    matchings: &mut Matchings<'a>,
) -> usize {
    let m = left_children.len();
    let n = right_children.len();

    let mut matrix_m = vec![vec![0; n + 1]; m + 1];
    let mut matrix_t = vec![vec![Entry::default(); n + 1]; m + 1];

    for i in 1..m + 1 {
        for j in 1..n + 1 {
            let left_child = left_children.get(i - 1).unwrap();
            let right_child = right_children.get(j - 1).unwrap();

            let w = crate::calculate_matchings(left_child, right_child);
            let matching = w
                .get_matching_entry(left_child, right_child)
                .unwrap_or_default();

            if matrix_m[i][j - 1] > matrix_m[i - 1][j] {
                if matrix_m[i][j - 1] > matrix_m[i - 1][j - 1] + matching.score {
                    matrix_m[i][j] = matrix_m[i][j - 1];
                    matrix_t[i][j] = Entry(Direction::Left, w);
                } else {
                    matrix_m[i][j] = matrix_m[i - 1][j - 1] + matching.score;
                    matrix_t[i][j] = Entry(Direction::Diag, w);
                }
            } else if matrix_m[i - 1][j] > matrix_m[i - 1][j - 1] + matching.score {
                matrix_m[i][j] = matrix_m[i - 1][j];
                matrix_t[i][j] = Entry(Direction::Top, w);
            } else {
                matrix_m[i][j] = matrix_m[i - 1][j - 1] + matching.score;
                matrix_t[i][j] = Entry(Direction::Diag, w);
            }
        }
    }

    let mut i = m;
    let mut j = n;

    while i >= 1 && j >= 1 {
        match matrix_t.get(i).unwrap().get(j).unwrap().0 {
            Direction::Top => i -= 1,
            Direction::Left => j -= 1,
            Direction::Diag => {
                if matrix_m[i][j] > matrix_m[i - 1][j - 1] {
                    matchings.extend(matrix_t[i][j].1.clone());
                }
                i -= 1;
                j -= 1;
            }
        }
    }

    matrix_m[m][n]
}
