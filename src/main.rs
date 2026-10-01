use ql_checker::TypeChecker;
use ql_codegen::{
    CodeGenerator, CpuBackend, CudaBackend, HipBackend, MetalBackend, OpenCLBackend,
};
use ql_lexer::Lexer;
use ql_parser::Parser;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "run" => {
            if args.len() < 3 {
                println!("[QLC ERROR] Missing input file for 'run' command.");
                return;
            }
            let file_path = &args[2];
            let target = parse_target_flag(&args[3..]);
            
            let exe_path = compile_file(file_path, None, &target);
            if let Some(exe) = exe_path {
                println!("\n--- Running Executable Output ---");
                let _ = Command::new(format!("./{}", exe)).status();
                let _ = fs::remove_file(exe); // Clean up temp binary on run
            }
        }
        "build" => {
            if args.len() < 3 {
                println!("[QLC ERROR] Missing input file for 'build' command.");
                return;
            }
            let file_path = &args[2];
            let custom_output = parse_output_flag(&args[3..]);
            let target = parse_target_flag(&args[3..]);

            compile_file(file_path, custom_output, &target);
        }
        _ => {
            // Fallback for direct usage: qlc <file.ql> [--target <gpu>]
            let custom_output = parse_output_flag(&args[2..]);
            let target = parse_target_flag(&args[2..]);
            compile_file(command, custom_output, &target);
        }
    }
}

fn print_help() {
    println!("QLang Compiler (qlc) - Version 0.1.0");
    println!("Usage:");
    println!("  qlc run <file.ql> [--target <device>]               Compile and immediately run program");
    println!("  qlc build <file.ql> [-o <name>] [--target <device>]  Build standalone binary executable");
    println!("  qlc <file.ql> [--target <device>]                   Default build behavior");
    println!("\nSupported Targets (--target / -t):");
    println!("  cpu    (default) Pure C / Native Loop Backend");
    println!("  opencl           Intel iGPU & OpenCL Backend");
    println!("  cuda             NVIDIA RTX / CUDA C API Backend");
    println!("  metal            Apple Silicon Metal Backend");
    println!("  hip              AMD Radeon / ROCm Backend");
}

fn parse_output_flag(args: &[String]) -> Option<String> {
    for i in 0..args.len() {
        if args[i] == "-o" && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
    }
    None
}

fn parse_target_flag(args: &[String]) -> String {
    for i in 0..args.len() {
        if (args[i] == "--target" || args[i] == "-t") && i + 1 < args.len() {
            return args[i + 1].to_lowercase();
        }
    }
    "cpu".to_string()
}

fn compile_file(file_path: &str, output_name: Option<String>, target: &str) -> Option<String> {
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("[QLC ERROR] Could not read file: {}", file_path);
    });

    println!("[QLC] Compiling '{}' for target '{}'...", file_path, target);

    // 1. Lexing
    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();

    // 2. Parsing
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    // 3. Type & Shape Checking
    let mut checker = TypeChecker::new();
    checker.check_program(&ast);

    // 4. Code Generation (Multi-Backend Support)
    let mut codegen = match target {
        "cuda" => CodeGenerator::with_backend(Box::new(CudaBackend::new())),
        "opencl" => CodeGenerator::with_backend(Box::new(OpenCLBackend::new())),
        "metal" => CodeGenerator::with_backend(Box::new(MetalBackend::new())),
        "hip" => CodeGenerator::with_backend(Box::new(HipBackend::new())),
        _ => CodeGenerator::with_backend(Box::new(CpuBackend::new())),
    };

    let c_code = codegen.generate(&ast, &checker);

    fs::write("output.c", &c_code).expect("Failed to write generated C code");

    // 5. Resolution & Execution via TinyCC / GCC
    let base_name = file_path.replace(".ql", "");
    let output_exe = match output_name {
        Some(name) => {
            if name.ends_with(".exe") {
                name
            } else {
                format!("{}.exe", name)
            }
        }
        None => format!("{}.exe", base_name),
    };

    let tcc_local = Path::new("tcc/tcc.exe");
    let (compiler_bin, compiler_name) = if tcc_local.exists() {
        (tcc_local.to_str().unwrap(), "Bundled TinyCC")
    } else {
        ("gcc.exe", "System GCC (Fallback)")
    };

    println!("[QLC] Building executable via {} ('{}')...", compiler_name, output_exe);

    let status = Command::new(compiler_bin)
        .arg("output.c")
        .arg("-o")
        .arg(&output_exe)
        .status();

    let _ = fs::remove_file("output.c"); // Cleanup C intermediate source

    match status {
        Ok(s) if s.success() => {
            println!("[QLC SUCCESS] Compiled successfully -> {}", output_exe);
            Some(output_exe)
        }
        _ => {
            println!("[QLC ERROR] Compilation failed using target compiler: {}", compiler_bin);
            None
        }
    }
}