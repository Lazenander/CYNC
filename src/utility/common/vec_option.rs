pub fn vo_to_ov<T>(vo: Vec<Option<T>>) -> Option<Vec<T>> {
    vo.into_iter().fold(Some(vec![]), |acc, x| match acc {
        Some(mut acc) => {
            acc.push(x?);
            Some(acc)
        }
        None => None,
    })
}
