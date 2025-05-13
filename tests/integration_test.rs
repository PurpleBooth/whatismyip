use std::process::Command;
use std::str;

// Helper function to run the program with given arguments and check the output
fn run_with_args(args: &[&str]) -> String {
    // Run the program with cargo run and the given arguments
    let output = Command::new("cargo")
        .args(["run", "--"])
        .args(args)
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute cargo run with args: {args:?}"));

    // Check that the program executed successfully
    assert!(
        output.status.success(),
        "Program execution failed with args: {args:?}"
    );

    // Convert the output to a string
    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");

    // Check that the output is not empty
    assert!(
        !stdout.trim().is_empty(),
        "Program output is empty with args: {args:?}"
    );

    // Check that the output contains at least one IP address
    let contains_ipv4 = stdout.contains('.');
    let contains_ipv6 = stdout.contains(':');

    assert!(
        contains_ipv4 || contains_ipv6,
        "Output does not contain an IP address with args: {args:?}\nOutput: {stdout}"
    );

    stdout.to_string()
}

#[test]
fn test_cargo_run() {
    // Run the program with no arguments
    let stdout = run_with_args(&[]);
    println!("Program output: {stdout}");
}

#[test]
fn test_cargo_run_with_only_local() {
    // Run the program with --only-local
    let stdout = run_with_args(&["--only-local"]);
    println!("Program output with --only-local: {stdout}");
}

#[test]
fn test_cargo_run_with_only_4() {
    // Run the program with --only-4
    let stdout = run_with_args(&["--only-4"]);
    println!("Program output with --only-4: {stdout}");

    // Verify that the output only contains IPv4 addresses (contains dots but no colons)
    assert!(
        stdout.contains('.'),
        "Output does not contain IPv4 addresses"
    );
    assert!(
        !stdout.contains(':'),
        "Output contains IPv6 addresses when it shouldn't"
    );

    // Run with both --only-4 and --only-local to test the match arm for IPv4-only
    let local_stdout = run_with_args(&["--only-4", "--only-local"]);
    println!("Program output with --only-4 --only-local: {local_stdout}");

    // Verify that the output only contains IPv4 addresses
    assert!(
        local_stdout.contains('.'),
        "Output does not contain IPv4 addresses with --only-local"
    );
    assert!(
        !local_stdout.contains(':'),
        "Output contains IPv6 addresses with --only-local when it shouldn't"
    );

    // The outputs should be different when using --only-local vs not using it
    // This helps catch the deletion of the match arm
    let wan_stdout = run_with_args(&["--only-4", "--only-wan"]);
    assert_ne!(
        local_stdout, wan_stdout,
        "Local and WAN outputs should be different for IPv4"
    );
}

#[test]
fn test_cargo_run_with_only_6() {
    // Run the program with --only-6
    let stdout = run_with_args(&["--only-6"]);
    println!("Program output with --only-6: {stdout}");

    // Verify that the output only contains IPv6 addresses (contains colons)
    assert!(
        stdout.contains(':'),
        "Output does not contain IPv6 addresses"
    );

    // Run with both --only-6 and --only-local to test the match arm for IPv6-only
    let local_stdout = run_with_args(&["--only-6", "--only-local"]);
    println!("Program output with --only-6 --only-local: {local_stdout}");

    // Verify that the output only contains IPv6 addresses
    assert!(
        local_stdout.contains(':'),
        "Output does not contain IPv6 addresses with --only-local"
    );
    assert!(
        !local_stdout.contains('.'),
        "Output contains IPv4 addresses with --only-local when it shouldn't"
    );
}

#[test]
fn test_local_and_wan_outputs_are_different() {
    // Test that local and WAN outputs have an empty intersection for IPv4
    let local_ipv4 = run_with_args(&["--only-4", "--only-local"]);
    let wan_ipv4 = run_with_args(&["--only-4", "--only-wan"]);
    // Convert outputs to sets of lines to handle different ordering
    let local_ipv4_lines: std::collections::HashSet<&str> = local_ipv4.lines().collect();
    let wan_ipv4_lines: std::collections::HashSet<&str> = wan_ipv4.lines().collect();

    // Find the intersection between local and WAN IPv4 addresses
    let intersection: std::collections::HashSet<_> =
        local_ipv4_lines.intersection(&wan_ipv4_lines).collect();
    assert!(
        intersection.is_empty(),
        "Local and WAN IPv4 outputs should have no IPs in common: {:?}",
        intersection
    );

    // Test that local and WAN outputs have an empty intersection for IPv6
    let local_ipv6 = run_with_args(&["--only-6", "--only-local"]);
    let wan_ipv6 = run_with_args(&["--only-6", "--only-wan"]);

    // Convert outputs to sets of lines to handle different ordering
    let local_ipv6_lines: std::collections::HashSet<&str> = local_ipv6.lines().collect();
    let wan_ipv6_lines: std::collections::HashSet<&str> = wan_ipv6.lines().collect();

    // Find the intersection between local and WAN IPv6 addresses
    let intersection: std::collections::HashSet<_> =
        local_ipv6_lines.intersection(&wan_ipv6_lines).collect();
    assert!(
        intersection.is_empty(),
        "Local and WAN IPv6 outputs should have no IPs in common: {:?}",
        intersection
    );
}

#[test]
fn test_cargo_run_with_only_wan() {
    // Run the program with --only-wan
    let stdout = run_with_args(&["--only-wan"]);
    println!("Program output with --only-wan: {stdout}");

    // This should include both IPv4 and IPv6 addresses
    let has_ipv4 = stdout.contains('.');
    let has_ipv6 = stdout.contains(':');

    // At least one type of IP should be present
    assert!(
        has_ipv4 || has_ipv6,
        "Output does not contain any IP addresses"
    );
}

#[test]
fn test_condition_not_only_6_and_not_only_local() {
    // This test specifically targets the condition !args.only_6 && !args.only_local
    // at line 61:25 in main.rs

    // First, get output with default settings (should include IPv4 WAN addresses)
    let default_stdout = run_with_args(&[]);

    // Then, get output with --only-6 (should NOT include IPv4 WAN addresses)
    let only_6_stdout = run_with_args(&["--only-6"]);

    // Verify that the default output contains IPv4 addresses
    assert!(
        default_stdout.contains('.'),
        "Default output does not contain IPv4 addresses"
    );

    // Verify that the --only-6 output does not contain IPv4 addresses
    assert!(
        !only_6_stdout.contains('.'),
        "Output with --only-6 contains IPv4 addresses when it shouldn't"
    );

    // The outputs should be different
    assert_ne!(
        default_stdout, only_6_stdout,
        "Default and --only-6 outputs should be different"
    );

    // Now test with --only-local (should NOT include IPv4 WAN addresses)
    let only_local_stdout = run_with_args(&["--only-local"]);

    // The outputs should be different
    assert_ne!(
        default_stdout, only_local_stdout,
        "Default and --only-local outputs should be different"
    );
}

#[test]
fn test_cargo_run_with_only_wan_and_only_4() {
    // Run the program with --only-wan and --only-4
    let stdout = run_with_args(&["--only-wan", "--only-4"]);
    println!("Program output with --only-wan and --only-4: {stdout}");

    // Verify that the output only contains IPv4 addresses
    assert!(
        stdout.contains('.'),
        "Output does not contain IPv4 addresses"
    );
    assert!(
        !stdout.contains(':'),
        "Output contains IPv6 addresses when it shouldn't"
    );
}

#[test]
fn test_cargo_run_with_only_wan_and_only_6() {
    // Run the program with --only-wan and --only-6
    let stdout = run_with_args(&["--only-wan", "--only-6"]);
    println!("Program output with --only-wan and --only-6: {stdout}");

    // Verify that the output only contains IPv6 addresses
    assert!(
        stdout.contains(':'),
        "Output does not contain IPv6 addresses"
    );
}

#[test]
fn test_cargo_run_with_reverse() {
    // Run the program with --reverse
    let stdout = run_with_args(&["--reverse"]);
    println!("Program output with --reverse: {stdout}");

    // Verify that the output contains parentheses, which indicate reverse DNS entries
    assert!(
        stdout.contains('('),
        "Output does not contain reverse DNS entries"
    );
}
