use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Pulse {
    High,
    Low,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct ModuleId(usize);

#[derive(Debug, Clone)]
pub enum Module {
    FlipFlop(bool, Vec<ModuleId>),
    Conjunction(HashMap<ModuleId, Pulse>, Vec<ModuleId>),
    Broadcast(Vec<ModuleId>),
    Button(Vec<ModuleId>),
}

impl Module {
    fn outputs(&self) -> &[ModuleId] {
        match self {
            Module::FlipFlop(_, out) => out,
            Module::Conjunction(_, out) => out,
            Module::Broadcast(out) => out,
            Module::Button(out) => out,
        }
    }
    fn notify_input(&mut self, input: ModuleId) {
        match self {
            Module::Conjunction(h, _) => {
                h.insert(input, Pulse::Low);
            }
            _ => {}
        }
    }
    fn send(
        &mut self,
        id: ModuleId,
        input: ModuleId,
        pulse: Pulse,
    ) -> Vec<(ModuleId, ModuleId, Pulse)> {
        let new_pulse = match self {
            Module::Button(_) | Module::Broadcast(_) => Some(pulse),
            Module::Conjunction(inputs, _) => {
                inputs.insert(input, pulse);
                if inputs.values().all(|p| *p == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            Module::FlipFlop(state, _) => match pulse {
                Pulse::High => None,
                Pulse::Low => {
                    let new_pulse = if *state { Pulse::Low } else { Pulse::High };
                    *state = !*state;
                    Some(new_pulse)
                }
            },
        };

        if let Some(new_pulse) = new_pulse {
            self.outputs().iter().map(|o| (id, *o, new_pulse)).collect()
        } else {
            vec![]
        }
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 20;
    type Input1 = (HashMap<String, ModuleId>, HashMap<ModuleId, Module>);
    type Input2 = (HashMap<String, ModuleId>, HashMap<ModuleId, Module>);
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        let mut names: HashMap<String, ModuleId> = HashMap::new();

        fn get_name(names: &mut HashMap<String, ModuleId>, name: &str) -> ModuleId {
            let len = names.len();
            *names
                .entry(name.to_string())
                .or_insert_with(|| ModuleId(len))
        }

        let mut modules: HashMap<ModuleId, Module> = s
            .lines()
            .map(|l| {
                let (name, rest) = l.split_once(" -> ").unwrap();
                let out: Vec<_> = rest
                    .split(',')
                    .map(|n| get_name(&mut names, n.trim()))
                    .collect();

                match name {
                    "broadcaster" => (get_name(&mut names, name), Module::Broadcast(out)),
                    v if v.starts_with('%') => {
                        let name = v.strip_prefix('%').unwrap();
                        (get_name(&mut names, name), Module::FlipFlop(false, out))
                    }
                    v if v.starts_with('&') => {
                        let name = v.strip_prefix('&').unwrap();
                        (
                            get_name(&mut names, name),
                            Module::Conjunction(HashMap::default(), out),
                        )
                    }
                    _ => panic!(),
                }
            })
            .collect();

        modules.insert(
            get_name(&mut names, "button"),
            Module::Button(vec![get_name(&mut names, "broadcaster")]),
        );

        let conns: Vec<_> = modules
            .iter()
            .map(|(k, v)| (*k, v.outputs().to_vec()))
            .collect();

        conns.iter().for_each(|(id, outs)| {
            for o in outs {
                if let Some(m) = modules.get_mut(o) {
                    m.notify_input(*id);
                }
            }
        });

        (names, modules)
    }
    fn process_input2(s: &str) -> Self::Input2 {
        Self::process_input1(s)
    }
    fn p1((ids, modules): &Self::Input1) -> Self::Sol1 {
        let mut modules = (*modules).clone();
        let button = *ids.get("button").unwrap();
        let broadcast = *ids.get("broadcaster").unwrap();
        let mut worklist = VecDeque::from([]);

        let names: HashMap<_, _> = ids.iter().map(|(k, v)| (v, k)).collect();

        let mut high = 0;
        let mut low = 0;
        for _ in 0..1000 {
            worklist.push_back((button, broadcast, Pulse::Low));

            while let Some((from, to, p)) = worklist.pop_front() {
                match p {
                    Pulse::High => high += 1,
                    Pulse::Low => low += 1,
                }

                // println!(
                //     "{} -{:?}-> {}",
                //     names.get(&from).unwrap(),
                //     p,
                //     names.get(&to).unwrap()
                // );
                if let Some(m) = modules.get_mut(&to) {
                    worklist.extend(m.send(to, from, p));
                }
            }
        }
        // println!("{:?}", modules);

        // println!("{:?}", (high, low));
        high * low
    }
    fn p2((ids, modules): &Self::Input2) -> Self::Sol2 {
        let mut modules = (*modules).clone();
        let button = *ids.get("button").unwrap();
        let broadcast = *ids.get("broadcaster").unwrap();
        let rx = *ids.get("rx").unwrap();
        let mut worklist = VecDeque::from([]);

        let names: HashMap<_, _> = ids.iter().map(|(k, v)| (v, k)).collect();

        let mut i = 0;
        loop {
            if i % 100000 == 0 {
                println!("{:?}", i);
            }
            i += 1;
            worklist.push_back((button, broadcast, Pulse::Low));

            while let Some((from, to, p)) = worklist.pop_front() {
                if to == rx && p == Pulse::Low {
                    return i;
                }

                // println!(
                //     "{} -{:?}-> {}",
                //     names.get(&from).unwrap(),
                //     p,
                //     names.get(&to).unwrap()
                // );
                if let Some(m) = modules.get_mut(&to) {
                    worklist.extend(m.send(to, from, p));
                }
            }
        }
    }
}

//crate::default_tests!((), ());
crate::string_tests!(
    [
        (
            foo_sol1,
            "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
            32000000
        ),
        (
            foo2_sol1,
            "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
            11687500
        )
    ],
    [(foo_sol2, "hi2", 1)]
);
