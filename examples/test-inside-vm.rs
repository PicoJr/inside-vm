use inside_vm::cpuid_cycle_count_avg;

fn main() {
    let avg = cpuid_cycle_count_avg(5, 100, 5);
    println!("avg cycles for __cpuid: {}", avg);
}
