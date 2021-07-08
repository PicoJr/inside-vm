use std::arch::x86_64::{_rdtsc, __cpuid, CpuidResult};

pub fn cpuid_cycle_count_avg((low, samples, high): (usize, usize, usize)) -> u64 {
    let mut tsc1: u64;
    let mut tsc2: u64;
    let mut cycles: Vec<u64> = vec![];
    let mut cpuid = CpuidResult{
        eax: 0,
        ebx: 0,
        ecx: 0,
        edx: 0
    };
    for _ in 0..(low + samples + high) {
        unsafe {
            tsc1 = _rdtsc();
            cpuid = __cpuid(0);
            tsc2 = _rdtsc();
        }
        cycles.push(tsc2 - tsc1);
    }
    cycles.sort_unstable();
    // remove low and high outliers, keep samples
    let cycles_without_outliers = &cycles[low..low + samples];
    let avg = cycles_without_outliers.iter().sum::<u64>() / std::cmp::max(samples as u64, 1);
    // disgusting hack to prevent compiler from optimizing __cpuid call
    avg + (cpuid.eax as u64 % 2)
}

pub fn inside_vm_threshold(threshold: u64) -> bool {
    cpuid_cycle_count_avg((5, 100, 5)) > threshold
}

pub fn inside_vm() -> bool {
    inside_vm_threshold(1_000)
}


#[cfg(test)]
mod tests {
    use crate::cpuid_cycle_count_avg;

    #[test]
    fn test_cpuid_cycle_count_avg() {
        let avg = cpuid_cycle_count_avg((5, 100, 5));
        assert!(avg < 1000);
    }
}
