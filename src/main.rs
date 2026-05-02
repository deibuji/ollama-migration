use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::Parser;
use ollama_migrator::cli::{Cli, Commands, OutputFormat};
use ollama_migrator::export::{MigrationJob, export_all, export_model};
use ollama_migrator::gguf::validate_gguf;
use ollama_migrator::ollama::OllamaInstallation;
use ollama_migrator::output::{JsonOutput, Output, TableOutput};
use ollama_migrator::paths::ensure_extension;
use ollama_migrator::platform::Platform;

fn main() -> ExitCode {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        return ExitCode::from(1);
    }

    ExitCode::SUCCESS
}

fn run(cli: Cli) -> Result<()> {
    let output: Box<dyn Output> = match cli.format {
        OutputFormat::Table => Box::new(TableOutput),
        OutputFormat::Json => Box::new(JsonOutput),
    };

    let ollama_dir = match cli.ollama_dir {
        Some(dir) => dir,
        None => Platform::current().ollama_default_dir()?,
    };

    match cli.command {
        Commands::List => {
            cmd_list(&ollama_dir, output.as_ref())?;
        }
        Commands::Export {
            models,
            output: out,
        } => {
            cmd_export(&ollama_dir, &models, out, output.as_ref())?;
        }
        Commands::ExportAll { output, pattern } => {
            cmd_export_all(&ollama_dir, &output, pattern.as_deref())?;
        }
        Commands::Info { target } => {
            cmd_info(&ollama_dir, &target, output.as_ref())?;
        }
        Commands::Verify { path } => {
            cmd_verify(&path)?;
        }
    }

    Ok(())
}

fn cmd_list(ollama_dir: &PathBuf, output: &dyn Output) -> Result<()> {
    let install = OllamaInstallation::discover_at(ollama_dir)
        .context("Failed to discover Ollama installation")?;

    if install.models.is_empty() {
        println!("No models found.");
        return Ok(());
    }

    output.write_model_list(&install.models);
    Ok(())
}

fn cmd_export(
    ollama_dir: &PathBuf,
    model_names: &[String],
    output: Option<PathBuf>,
    out: &dyn Output,
) -> Result<()> {
    let install = OllamaInstallation::discover_at(ollama_dir)
        .context("Failed to discover Ollama installation")?;

    let mut success = 0;
    let mut failed = 0;

    for name in model_names {
        let model = install
            .find_model(name)
            .ok_or_else(|| anyhow::anyhow!("Model not found: {}", name))?;

        let dest = output
            .as_ref()
            .map(|p| p.clone())
            .unwrap_or_else(|| PathBuf::from(format!("{}.gguf", name.replace('/', "_"))));

        let dest = ensure_extension(&dest, "gguf");
        let job = MigrationJob::new(model.clone(), dest.clone());

        match export_model(model, &dest, job) {
            Ok(()) => {
                println!("Exported: {} → {}", model.name, dest.display());
                success += 1;
            }
            Err(e) => {
                eprintln!("Failed to export {}: {}", model.name, e);
                failed += 1;
            }
        }
    }

    out.write_export_summary(success, failed);

    if failed > 0 {
        anyhow::bail!("{} export(s) failed", failed);
    }

    Ok(())
}

fn cmd_export_all(ollama_dir: &PathBuf, output_dir: &PathBuf, pattern: Option<&str>) -> Result<()> {
    let install = OllamaInstallation::discover_at(ollama_dir)
        .context("Failed to discover Ollama installation")?;

    let models: Vec<_> = if let Some(pat) = pattern {
        install
            .models
            .into_iter()
            .filter(|m| m.name.contains(pat))
            .collect()
    } else {
        install.models
    };

    if models.is_empty() {
        println!("No models to export.");
        return Ok(());
    }

    std::fs::create_dir_all(output_dir)?;

    let summary = export_all(&models, output_dir, None);

    println!(
        "Export complete: {}/{} succeeded",
        summary.success_count(),
        summary.jobs.len()
    );

    if summary.failure_count() > 0 {
        anyhow::bail!("{} export(s) failed", summary.failure_count());
    }

    Ok(())
}

fn cmd_info(ollama_dir: &PathBuf, target: &str, output: &dyn Output) -> Result<()> {
    // Check if target is a file path
    if std::path::Path::new(target).exists() {
        let header =
            ollama_migrator::gguf::GGUFHeader::from_file(&std::path::PathBuf::from(target))?;
        output.write_gguf_info(&header.try_into()?);
        return Ok(());
    }

    let install = OllamaInstallation::discover_at(ollama_dir)
        .context("Failed to discover Ollama installation")?;

    let model = install
        .find_model(target)
        .ok_or_else(|| anyhow::anyhow!("Model not found: {}", target))?;

    output.write_model_info(model);
    Ok(())
}

fn cmd_verify(path: &PathBuf) -> Result<()> {
    let result = validate_gguf(path).context("Failed to validate GGUF file")?;

    if result.is_valid() {
        println!("Valid GGUF file");
        Ok(())
    } else {
        eprintln!("Validation failed:");
        for error in result.errors {
            eprintln!("  - {}", error);
        }
        anyhow::bail!("Invalid GGUF file");
    }
}
