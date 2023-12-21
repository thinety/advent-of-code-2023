use std::collections::{HashMap, VecDeque};

enum Module {
    Broadcaster {
        destinations: Vec<(usize, usize)>,
    },
    FlipFlop {
        on: bool,
        destinations: Vec<(usize, usize)>,
    },
    Conjunction {
        last: Vec<Pulse>,
        destinations: Vec<(usize, usize)>,
    },
    Dummy,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

fn parse(input: &str) -> (usize, Option<usize>, Vec<Module>) {
    let mut broadcaster = None;
    let mut rx = None;
    let mut modules = Vec::new();

    let mut mapping = HashMap::new();
    for line in input.lines() {
        let (module, _) = line.split_once(" -> ").unwrap();

        if module == "broadcaster" {
            modules.push(Module::Broadcaster {
                destinations: Vec::new(),
            });
            assert!(broadcaster.replace(modules.len() - 1).is_none());
            continue;
        }

        let (r#type, name) = module.split_at(1);
        modules.push(match r#type {
            "%" => Module::FlipFlop {
                on: false,
                destinations: Vec::new(),
            },
            "&" => Module::Conjunction {
                last: Vec::new(),
                destinations: Vec::new(),
            },
            _ => panic!("invalid node type"),
        });
        mapping.insert(name, modules.len() - 1);
    }
    let broadcaster = broadcaster.unwrap();

    for line in input.lines() {
        let (module, destinations) = line.split_once(" -> ").unwrap();

        let destinations = destinations
            .split(", ")
            .map(|destination| {
                let i = *mapping.entry(destination).or_insert_with(|| {
                    modules.push(Module::Dummy);
                    modules.len() - 1
                });
                if destination == "rx" {
                    assert!(rx.replace(i).is_none());
                }
                let j = match &mut modules[i] {
                    Module::Broadcaster { .. } | Module::FlipFlop { .. } | Module::Dummy => 0,
                    Module::Conjunction { last, .. } => {
                        last.push(Pulse::Low);
                        last.len() - 1
                    }
                };
                (i, j)
            })
            .collect();

        let i = if module == "broadcaster" {
            broadcaster
        } else {
            let (_, name) = module.split_at(1);
            mapping[name]
        };

        match &mut modules[i] {
            Module::Broadcaster { destinations: d } => *d = destinations,
            Module::FlipFlop {
                on: _,
                destinations: d,
            } => *d = destinations,
            Module::Conjunction {
                last: _,
                destinations: d,
            } => *d = destinations,
            Module::Dummy => unreachable!(),
        }
    }

    (broadcaster, rx, modules)
}

pub fn part1(input: &str) -> u32 {
    let (broadcaster, _, mut modules) = parse(input);

    let mut low = 0;
    let mut high = 0;

    let mut queue = VecDeque::new();
    for _ in 0..1000 {
        queue.push_back((Pulse::Low, (broadcaster, 0)));
        while let Some((pulse, (i, j))) = queue.pop_front() {
            match pulse {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            }
            match &mut modules[i] {
                Module::Broadcaster { destinations } => {
                    for destination in destinations {
                        queue.push_back((pulse, *destination));
                    }
                }
                Module::FlipFlop { on, destinations } => match pulse {
                    Pulse::Low => {
                        *on = !*on;
                        let new_pulse = if *on { Pulse::High } else { Pulse::Low };
                        for destination in destinations {
                            queue.push_back((new_pulse, *destination));
                        }
                    }
                    Pulse::High => {}
                },
                Module::Conjunction { last, destinations } => {
                    last[j] = pulse;
                    let new_pulse = if last.iter().all(|&pulse| pulse == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for destination in destinations {
                        queue.push_back((new_pulse, *destination));
                    }
                }
                Module::Dummy => {}
            }
        }
    }

    low * high
}

crate::samples! {
    (part1_sample1, part1, "sample1.in", 32000000),
    (part1_sample2, part1, "sample2.in", 11687500),
    (part1_puzzle, part1, "puzzle.in", 839775244),
}
