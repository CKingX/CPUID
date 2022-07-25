use std::fmt::Write;

fn main() {

    let mut output = String::new();

    // Get from https://github.com/InstLatx64/InstLatx64/tree/e833cd79ce0aab79df0d2879b14e01d4edd359b7/GenuineIntel
    let full_cpuid_1 = "000906EA-00100800-7FFAFBBF-BFEBFBFF";
    let full_cpuid_7 = "00000000-029C67AF-40000000-9C000000";
    let (_,_,ecx,edx) = parse(full_cpuid_1);

    for feature in X86Features1ECX::iterator() {
        let result = test_feature(ecx, feature as u32);
        _ = writeln!(&mut output, "{:?}: {result}", feature);
    }

    for feature in X86Features1EDX::iterator() {
        let result = test_feature(edx, feature as u32);
        _ = writeln!(&mut output, "{:?}: {result}", feature);
    }

    let (_,ebx,ecx,edx) = parse(full_cpuid_7);

    for feature in X86Features70EBX::iterator() {
        let result = test_feature(ebx, feature as u32);
        _ = writeln!(&mut output, "{:?}: {result}", feature);
    }

    for feature in X86Features70EDX::iterator() {
        let result = test_feature(edx, feature as u32);
        _ = writeln!(&mut output, "{:?}: {result}", feature);
    }

    for feature in X86Features70ECX::iterator() {
        let result = test_feature(ecx, feature as u32);
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
    if result == true {"✅"} else {"❌"}
}

fn parse(cpuid: &str) -> (&str,&str,&str,&str) {
    let mut cpuid = cpuid.split('-');
    let eax = cpuid.next().unwrap();
    let ebx = cpuid.next().unwrap();
    let ecx = cpuid.next().unwrap();
    let edx = cpuid.next().unwrap();
    (eax,ebx,ecx,edx)
}

#[allow(non_camel_case_types)]
#[derive(Copy,Clone,Debug)]
enum X86Features1ECX {
    sse3         = 1 << 0, 
    pclmulqdq    = 1 << 1,    
    ssse3        = 1 << 9,  
    fma          = 1 << 12,
    sse4_1       = 1 << 19, 
    sse4_2       = 1 << 20, 
    movbe        = 1 << 22, 
    popcnt       = 1 << 23, 
    aes          = 1 << 25, 
    xsave        = 1 << 26,  
    avx          = 1 << 28,
    rdrnd        = 1 << 30,
    cmpxchg16b   = 1 << 13,
}

impl X86Features1ECX {
    pub fn iterator() -> [X86Features1ECX;13] {
        [X86Features1ECX::sse3,X86Features1ECX::pclmulqdq,X86Features1ECX::ssse3,X86Features1ECX::fma,X86Features1ECX::sse4_1,X86Features1ECX::sse4_2,
        X86Features1ECX::movbe,X86Features1ECX::popcnt,X86Features1ECX::aes,X86Features1ECX::xsave,X86Features1ECX::avx,X86Features1ECX::rdrnd,X86Features1ECX::cmpxchg16b]
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy,Clone,Debug)]
enum X86Features1EDX {
    sse     = 1 << 25,
    sse2    = 1 << 26,
    fxsr    = 1 << 24,
}

impl X86Features1EDX {
    pub fn iterator() -> [X86Features1EDX;3] {
        [X86Features1EDX::sse,X86Features1EDX::sse2, X86Features1EDX::fxsr]
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy,Clone,Debug)]
enum X86Features70EBX {
    adx         = 1 << 19,
    avx2        = 1 << 5,
    avx512_f    = 1 << 16,
    avx512_cd   = 1 << 28,
    bmi1        = 1 << 3,
    bmi2        = 1 << 8,
    sha         = 1 << 29,
    smep        = 1 << 7,
    rdseed      = 1 << 18,
    avx512_bw   = 1 << 30,
    avx512_dq   = 1 << 17,
}

impl X86Features70EBX {
    pub fn iterator() -> [X86Features70EBX;11] {
        [X86Features70EBX::adx,X86Features70EBX::avx512_cd,X86Features70EBX::avx2,X86Features70EBX::avx512_f,X86Features70EBX::bmi1,X86Features70EBX::bmi2,X86Features70EBX::sha,
        X86Features70EBX::smep,X86Features70EBX::rdseed,X86Features70EBX::avx512_bw,X86Features70EBX::avx512_dq]
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy,Clone,Debug)]
enum X86Features70EDX {
    cet_ibt     = 1 << 20,
}

impl X86Features70EDX {
    pub fn iterator() -> [X86Features70EDX;1] {
        [X86Features70EDX::cet_ibt]
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy,Clone,Debug)]
enum X86Features70ECX {
    cet_ss          = 1 << 7,
    avx512_bitalg   = 1 << 12,
}

impl X86Features70ECX {
    pub fn iterator() -> [X86Features70ECX;2] {
        [X86Features70ECX::cet_ss, X86Features70ECX::avx512_bitalg]
    }
}