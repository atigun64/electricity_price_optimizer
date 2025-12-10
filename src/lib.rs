use crate::{environment_data::EnvironmentData, schedule::Schedule};

pub mod action;
mod schedule;
mod prognoses;
mod environment_data;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn optimize(data: EnvironmentData) -> Schedule {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
