pub mod context;
pub mod job;
pub mod scheduler;
mod time_event;
mod timer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
