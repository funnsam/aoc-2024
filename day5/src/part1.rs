use crate::common::*;

pub fn solve((rules, order): Input<'_>) -> usize {
    order.into_iter()
        .filter_map(|ord| incorrect_pos(&rules, &ord).is_none().then_some(ord[ord.len() / 2]))
        .sum::<usize>()
}
