#[derive(Default, Debug)]
pub struct CheckersCache {
    moves: Vec<Option<Vec<(Option<usize>, usize)>>>,
}

impl CheckersCache {
    pub fn get(&self, position: usize) -> &Option<Vec<(Option<usize>, usize)>> {
        &self.moves[position]
    }

    pub fn update(
        &mut self,
        updates: impl Iterator<Item = (usize, Option<Vec<(Option<usize>, usize)>>)>,
    ) {
        for (i, value) in updates {
            self.moves[i] = value;
        }
    }
}
