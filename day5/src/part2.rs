use crate::common::*;

pub fn solve((rules, order): Input<'_>) -> usize {
    order.into_iter()
        .filter_map(|mut ord| {
            let mut mark = false;

            while let Some((a, b)) = incorrect_pos(&rules, &ord) {
                mark = true;
                ord.swap(a, b);
            }

            mark.then_some(ord[ord.len() / 2])
        })
        .sum::<usize>()
}
