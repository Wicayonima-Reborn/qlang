use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use ql_lexer::Lexer;
use ql_parser::Parser;
use ql_checker::TypeChecker;
use ql_codegen::CodeGenerator;

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
            let exe_path = compile_file(file_path, None);
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
            let mut custom_output = None;

            // Parse -o flag
            for i in 3..args.len() {
                if args[i] == "-o" && i + 1 < args.len() {
                    custom_output = Some(args[i + 1].clone());
                    break;
                }
            }

            compile_file(file_path, custom_output);
        }
        _ => {
            // Fallback for direct usage: qlc <file.ql>
            compile_file(command, None);
        }
    }
}

fn print_help() {
    println!("QLang Compiler (qlc) - Version 0.1.0");
    println!("Usage:");
    println!("  qlc run <file.ql>                 Compile and immediately run program");
    println!("  qlc build <file.ql> [-o <name>]   Build standalone binary executable");
    println!("  qlc <file.ql>                     Default build behavior");
}

fn compile_file(file_path: &str, output_name: Option<String>) -> Option<String> {
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| {
        panic!("[QLC ERROR] Could not read file: {}", file_path);
    });

    println!("[QLC] Compiling '{}'...", file_path);

    // 1. Lexing
    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();

    // 2. Parsing
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    // 3. Type & Shape Checking
    let mut checker = TypeChecker::new();
    checker.check_program(&ast);

    // 4. Code Generation
    let mut codegen = CodeGenerator::new();
    let c_code = codegen.generate(&ast, &checker);

    fs::write("output.c", &c_code).expect("Failed to write generated C code");

    // 5. Resolution & Execution via TinyCC
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