use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Dag(u64);

impl Dag {
    pub fn to_u64(&self) -> u64 {
        self.0
    }

    pub fn as_tuple(&self) -> Tuple {
        Tuple(self.to_u64())
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Tuple(u64);

impl Tuple {
    pub fn to_u64(&self) -> u64 {
        self.0
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Task(u32);

impl Task {
    pub fn to_u32(&self) -> u32 {
        self.0
    }
}

/// Track tuple acks across multiple tasks with minimal memory
/// requirements. The algorithm is from the stream-processing system
/// Storm.
///
/// The only allocation needed is two 64-bit values per task (task_id, ack value)
/// and scales to 2^64 tuples.
///
/// To track a tuple, you call `tuple`. The tuple's id will be XORed with the previous
/// ack value. As tuples are acked (marked as arbitrarily completed), the ack value will
/// once again be XORed. Once all tuples are acked/completed, the ack value will be 0.
#[derive(PartialEq, Eq, Debug)]
pub struct Ackr {
    buckets: HashMap<Dag, (Task, Tuple)>
}

impl Ackr {
    /// Create a new Ackr with no buckets/tasks.
    pub fn new() -> Ackr {
        Ackr {
            buckets: HashMap::new()
        }
    }

    /// Insert a new bucket entry with the Dag id as the initial ack value.
    pub fn insert(&mut self, dag_id: Dag, task_id: Task) {
        self.buckets.insert(dag_id, (task_id, dag_id.as_tuple()));
    }

    /// Add a tuple to the Dag's ack value. This is essentially just the first
    /// XOR.
    pub fn add_tuple(&mut self, dag_id: Dag, tuple_id: Tuple) {
        self.ack(dag_id, tuple_id);
    }

    /// XOR the ack value for a given Dag and the result is the new ack value.
    /// Acking once adds the tuple to the Dag, acking it twice removes it.
    pub fn ack(&mut self, dag_id: Dag, tuple_id: Tuple) -> Option<()> {
        if let Some(&mut (_, Tuple(ref mut x))) = self.buckets.get_mut(&dag_id) {
            *x ^= tuple_id.to_u64();

            Some(())
        } else {
            None
        }
    }

    pub fn get(&mut self, dag_id: Dag) -> Tuple {
        self.buckets[&dag_id].1
    }

    pub fn has_completed(&mut self, dag_id: Dag) -> bool {
        self.buckets[&dag_id].1.to_u64() == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let mut ackr = Ackr::new();
        ackr.insert(Dag(0x01), Task(0x02));
        assert_eq!(ackr.get(Dag(0x01)), Tuple(0x01));
    }

    #[test]
    fn ack() {
        let mut ackr = Ackr::new();
        ackr.insert(Dag(0x01), Task(0x01));
        ackr.ack(Dag(0x01), Tuple(0x01));
        assert_eq!(ackr.has_completed(Dag(0x01)), true);
    }

    #[test]
    fn ack_2() {
        let mut ackr = Ackr::new();

        // Dag id, task id
        ackr.insert(Dag(0x01), Task(1));

        ackr.add_tuple(Dag(0x01), Tuple(0x03));
        ackr.add_tuple(Dag(0x01), Tuple(0x04));
        ackr.ack(Dag(0x01), Tuple(0x04));
        ackr.ack(Dag(0x01), Tuple(0x03));
        assert_eq!(ackr.get(Dag(0x01)), Tuple(0x01));
        assert_eq!(ackr.has_completed(Dag(0x01)), false);
    }
}
