use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input: Vec<String> = input.iter().map(|text| String::from(*text)).collect();
    let input = Arc::new(Mutex::new(input));

    (0..worker_count)
        .into_iter()
        .map(|_index| {
            let closure = thread_closure(Arc::clone(&input));
            thread::spawn(closure)
        })
        .collect::<Vec<JoinHandle<HashMap<char, usize>>>>()
        .drain(..)
        .fold(HashMap::new(), |mut output, join_handle| {
            let thread_output = join_handle.join().unwrap();
            thread_output
                .into_iter()
                .for_each(|(character, thread_count)| {
                    let main_count = output.entry(character).or_default();
                    *main_count += thread_count;
                });
            output
        })
}

fn thread_closure(input: Arc<Mutex<Vec<String>>>) -> impl Fn() -> HashMap<char, usize> {
    move || {
        let mut output = HashMap::new();
        while let Some(string) = obtain_string(&input) {
            string
                .chars()
                .filter(|character| character.is_alphabetic())
                .flat_map(|character| character.to_lowercase())
                .for_each(|character| {
                    let count = output.entry(character).or_default();
                    *count += 1;
                });
        }
        output
    }
}

fn obtain_string(input: &Mutex<Vec<String>>) -> Option<String> {
    let mut input = input.lock().unwrap();
    input.pop()
}
