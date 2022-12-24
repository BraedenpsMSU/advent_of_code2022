use std::collections::hash_map::Values;
use std::collections::VecDeque;

pub trait ArrayIndexOne {
    type Value;

    fn get_dim(&self) -> (usize, usize);

    fn get_value(&self, position: &(usize, usize)) -> Self::Value;

    fn point_inbounds(&self, position: &(i64, i64)) -> bool {
        let dimension = self.get_dim();
        let p_pos = self.make_positive(position, &dimension);
        (1 <= p_pos.0) && (p_pos.0 <= dimension.0 as i64)
            && (1 <= p_pos.1) && (p_pos.1 <= dimension.1 as i64)
    }

    fn take_value(&self, position: &(i64, i64)) -> Option<Self::Value> {
        let p_pos = self.make_positive(position, &self.get_dim());
        let dim = self.get_dim();
        if (1 <= p_pos.0) && (p_pos.0 <= dim.0 as i64)
            && (1 <= p_pos.1) && (p_pos.1 <= dim.1 as i64)
        {Some(self.get_value(&(p_pos.0 as usize-1,p_pos.1 as usize-1)))}
        else {None}
    }

    fn make_positive(&self, position: &(i64, i64), dimension: &(usize, usize)) -> (i64, i64) {
        let x_value = if position.0 > 0 {position.0} else {dimension.0 as i64 + position.0 +1};
        let y_value = if position.1 > 0 {position.1} else {dimension.1 as i64 + position.1 +1};
        return (x_value, y_value)
    }
}

pub fn set_value_in_vec_deque<S,T>(
    array: &S,
    o_memo: &mut VecDeque<VecDeque<T>>,
    position: &(i64, i64),
    new_value: T)
where
    S: ArrayIndexOne,
    T: Clone
{
    let p_pos = array.make_positive(position, &array.get_dim());
    o_memo[p_pos.0 as usize - 1][p_pos.1 as usize - 1] = new_value.clone()
}

pub fn get_value_in_vec_deque<S,T>(
    array: &S,
    o_array: &VecDeque<VecDeque<T>>,
    position: &(i64, i64)) -> Option<T>
    where
        S: ArrayIndexOne,
        T: Clone
{
    let p_pos = array.make_positive(position, &array.get_dim());
    let dim = array.get_dim();
    if (1 <= p_pos.0) && (p_pos.0 <= dim.0 as i64)
        && (1 <= p_pos.1) && (p_pos.1 <= dim.1 as i64)
    {Some(o_array[p_pos.0 as usize - 1][p_pos.1 as usize - 1].clone())}
    else {None}
}

fn make_const_array<T: Clone>(dim: &(usize, usize), default: &T) -> VecDeque<VecDeque<T>> {
    let mut array: VecDeque<VecDeque<T>> = VecDeque::new();
    for _ in 0..dim.0 {
        let mut inner_array: VecDeque<T> = VecDeque::new();
        for _ in 0..dim.1{
            inner_array.push_back(default.clone());
        }
        array.push_back(inner_array);
    }
    array
}

pub fn get_final_positions<T: ArrayIndexOne>(array: &T, direction: &(i64, i64)) -> VecDeque<(i64, i64)>
    {get_init_positions(array, &(-direction.0, -direction.1))}

pub fn get_init_positions<T: ArrayIndexOne>(array: &T, direction: &(i64, i64)) -> VecDeque<(i64, i64)>
{
    let mut init_position: VecDeque<(i64, i64)> = VecDeque::new();
    let x_size = direction.0.abs() as u64;
    let x_sign = if direction.0.signum() == 0 { 1 } else { direction.0.signum() };
    let y_size = direction.1.abs() as u64;
    let y_sign = if direction.1.signum() == 0 { 1 } else { direction.1.signum() };
    let height = array.get_dim().0;
    let width = array.get_dim().1;
    for x_off in 1..(x_size + 1) {
        for index in 1..(width as u64 + 1) {
            init_position.push_back(
                array.make_positive(
                    &(x_sign * (x_off as i64), index as i64), &array.get_dim()
                ))
        }
    }
    for y_off in 1..(y_size + 1) {
        for index in (x_size + 1)..(height as u64 + 1) {
            init_position.push_back(
                array.make_positive(
                    &(x_sign * (index as i64), y_sign * y_off as i64), &array.get_dim()
                ))
        }
    }
    init_position
}

pub fn sweep_for_memo_direction<S,T>(array: &S,
                            direction: (i64, i64),
                            default: &T,
                            initializer: fn(&mut VecDeque<VecDeque<T>>, &S,
                                            &(i64, i64), &(i64, i64)),
                            updater: fn(&mut VecDeque<VecDeque<T>>, &S,
                                        &(i64, i64), &(i64, i64), &(i64, i64)) )
    -> VecDeque<VecDeque<T>>
    where
        S: ArrayIndexOne,
        T: Clone,
{
    let mut output = make_const_array(&array.get_dim(), default);
    let mut initial_positions = get_init_positions(array, &direction);
    for i in 0..initial_positions.len() {
                               initializer(&mut output, array, &initial_positions[i], &direction)
    }
    while !initial_positions.is_empty() {
        let mut prior_pos = initial_positions.pop_front().unwrap().clone();
        let next_pos = (prior_pos.0 + direction.0, prior_pos.1 + direction.1);
        let next_value = array.take_value(&next_pos);
        if next_value.is_none() {continue}
        else {initial_positions.push_back(next_pos)}
        updater(&mut output, &array, &prior_pos, &next_pos , &direction)
    }
    output
}