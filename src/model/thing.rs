trait Thing {}

struct HeapObject {}

struct ObjectRef {
    id: u64,
}

impl Thing for HeapObject {}

impl Thing for ObjectRef {}

impl Thing for Value {}