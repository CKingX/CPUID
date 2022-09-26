use std::fmt::Write;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

fn main() {
    let mut output = String::new();
    let mut args = std::env::args().skip(1);

    // Get from https://github.com/InstLatx64/InstLatx64/tree/e833cd79ce0aab79df0d2879b14e01d4edd359b7/GenuineIntel
    let full_cpuid_1 = args.next().unwrap();
    let full_cpuid_7 = args.next().unwrap();
    let full_cpuid_13_1 = args.next().unwrap();
    let full_cpuid_801 = args.next().unwrap();
    let (_, _, ecx, edx) = parse(&full_cpuid_1);

    for feature in X86Features1ECX::iter() {
        let result = test_feature(ecx, feature as u32);
        _ = writeln!(&mut output, "{:?}: {result}", feature);
    }

    for feature in X86Features1EDX::iter() {
        let result = test_feature(edx, feature as u32);
        _ = writeln!(&mut output, "{:?}: {result}", feature);
    }

    let (eax, ebx, ecx, edx) = parse(&full_cpuid_7);

    for feature in X86Features70EBX::iter() {
        let result = test_feature(ebx, feature as u32);
        _ = writeln!(&mut output, "{:?}: {result}", feature);
    }

    for feature in X86Features70EDX::iter() {
        let result = test_feature(edx, feature as u32);
        _ = writeln!(&mut output, "{:?}: {result}", feature);
    }

    for feature in X86Features70ECX::iter() {
        let result = test_feature(ecx, feature as u32);
        _ = writeln!(&mut output, "{:?}: {result}", feature);
    }

    for feature in X86Features70EAX::iter() {
        let result = test_feature(eax, feature as u32);
        _ = writeln!(&mut output, "{:?}: {result}", feature);
    }

    let (eax, _, _, _) = parse(&full_cpuid_13_1);

    for feature in X86Features13_1EAX::iter() {
        let result = test_feature(eax, feature as u32);
        _ = writeln!(&mut output, "{:?}: {result}", feature);
    }

    let (_, _, ecx, edx) = parse(&full_cpuid_801);
    for feature in X86Features80000001_ECX::iter() {
        let result = test_feature(ecx, feature as u32);
        _ = writeln!(&mut output, "{:?}: {result}", feature);
    }

    for feature in X86Features80000001_EDX::iter() {
        let result = test_feature(edx, feature as u32);
        _ = writeln!(&mut output, "{:?}: {result}", feature);
    }

    let mut output: Vec<_> = output.lines().collect();
    output.sort();
    for feature in output {
        println!("{feature}");
    }
}

fn test_feature(cpuid: &str, feature: u32) -> &str {
    let result = (u32::from_str_radix(cpuid, 16).unwrap() & feature) > 0;
    if result {
        "✅"
    } else {
        "❌"
    }
}

fn parse(cpuid: &str) -> (&str, &str, &str, &str) {
    let mut cpuid = cpuid.split('-');
    let eax = cpuid.next().unwrap();
    let ebx = cpuid.next().unwrap();
    let ecx = cpuid.next().unwrap();
    let edx = cpuid.next().unwrap();
    (eax, ebx, ecx, edx)
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, EnumIter, EnumString)]
enum X86Features1ECX {
    sse3 = 1 << 0,
    pclmulqdq = 1 << 1,
    ssse3 = 1 << 9,
    fma = 1 << 12,
    sse4_1 = 1 << 19,
    sse4_2 = 1 << 20,
    movbe = 1 << 22,
    popcnt = 1 << 23,
    aes = 1 << 25,
    xsave = 1 << 26,
    avx = 1 << 28,
    rdrnd = 1 << 30,
    cmpxchg16b = 1 << 13,
    f16c = 1 << 29,
    est_speedstep = 1 << 7,
    smx_trusted_execution = 1 << 6,
    virtualization = 1 << 5,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, EnumIter, EnumString)]
enum X86Features1EDX {
    sse = 1 << 25,
    sse2 = 1 << 26,
    fxsr = 1 << 24,
    mmx = 1 << 23,
    hyperthreading_smt = 1 << 28,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, EnumIter, EnumString)]
enum X86Features70EAX {
    lam = 1 << 26,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, EnumIter, EnumString)]
enum X86Features70EBX {
    adx = 1 << 19,
    avx2 = 1 << 5,
    avx512_f = 1 << 16,
    avx512_cd = 1 << 28,
    bmi1 = 1 << 3,
    bmi2 = 1 << 8,
    sha = 1 << 29,
    smep = 1 << 7,
    rdseed = 1 << 18,
    avx512_bw = 1 << 30,
    avx512_dq = 1 << 17,
    sgx = 1 << 2,
    avx512_ifma = 1 << 21,
    avx512_vl = 1 << 31,
    mpx = 1 << 14,
    ermsb = 1 << 9,
    smap = 1 << 20,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, EnumIter, EnumString)]
enum X86Features70EDX {
    cet_ibt = 1 << 20,
    avx512_vp2intersect = 1 << 8,
    serialize = 1 << 14,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, EnumIter, EnumString)]
enum X86Features70ECX {
    cet_ss = 1 << 7,
    avx512_bitalg = 1 << 12,
    avx512_vbmi = 1 << 1,
    avx512_vbmi2 = 1 << 6,
    avx512_vnni = 1 << 11,
    avx512_vpopcntdq = 1 << 14,
    avx512_gfni = 1 << 8,
    avx512_vaes = 1 << 9,
    avx512_vpclmulqdq = 1 << 10,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, EnumIter, EnumString)]
enum X86Features13_1EAX {
    xsaveopt = 1 << 0,
    xsavec = 1 << 1,
    xsaves = 1 << 3,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, EnumIter, EnumString)]
enum X86Features80000001_ECX {
    abm_popcnt_lzcnt = 1 << 5,
    sse4a = 1 << 6,
    skinit = 1 << 12,
    amd_v = 1 << 2,
    lahf_lm = 1 << 0,
    prefetch = 1 << 8,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, EnumIter, EnumString)]
enum X86Features80000001_EDX {
    pae = 1 << 6,
    nx = 1 << 20,
    x64 = 1 << 29,
}