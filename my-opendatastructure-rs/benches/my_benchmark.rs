#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use my_opendatastructure_rs::my_structure::naive_implementation::MyList;
use my_opendatastructure_rs::my_structure::List;

fn naive_implementation(c: &mut Criterion) {
    let my_list = MyList::new(vec![1, 2, 3]);
    c.bench_function("naieve_implementaiton_list_size", move |b| {
        b.iter(|| my_list.size())
    });

    let my_list = MyList::new(vec![1, 2, 3]);
    c.bench_function("naieve_implementaiton_list_get", move |b| {
        b.iter(|| my_list.get(1))
    });

    let mut my_list = MyList::new(vec![1, 2, 3]);
    c.bench_function("naieve_implementaiton_list_set", move |b| {
        b.iter(|| my_list.set(1, 5))
    });

    let mut my_list = MyList::new(vec![1, 2, 3]);
    c.bench_function("naieve_implementaiton_list_add", move |b| {
        b.iter(|| my_list.add(2, 5))
    });

    let mut my_list = MyList::new(vec![1, 2, 3]);
    c.bench_function("naieve_implementaiton_list_remove", move |b| {
        b.iter(|| {
            my_list.add(1, 1);
            my_list.remove(1)
        })
    });
}

criterion_group!(benches, naive_implementation);
criterion_main!(benches);
