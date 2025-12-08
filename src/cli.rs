use crate::client::GithubClient;
use crate::error::Result;
use crate::models::{CreateLabelRequest, UpdateLabelRequest};
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "biao")]
#[command(about = "GitHub label management CLI", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authenticate with GitHub (wrapper around `gh auth`)
    Auth {
        #[command(subcommand)]
        subcommand: Option<AuthSubcommands>,
    },

    /// List all labels
    List,

    /// Get a specific label
    Get { name: String },

    /// Create a new label
    Create {
        /// Label name
        name: String,

        /// Label color (hex without #, e.g., "ff0000")
        color: String,

        /// Optional description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// Update an existing label
    Update {
        /// Label name to update
        name: String,

        /// New label name
        #[arg(long)]
        new_name: Option<String>,

        /// New color (hex without #)
        #[arg(long)]
        color: Option<String>,

        /// New description
        #[arg(long)]
        description: Option<String>,
    },

    /// Delete a label
    Delete {
        /// Label name to delete
        name: String,

        /// Skip confirmation
        #[arg(short)]
        force: bool,
    },

    /// Apply label changes from a TOML config file
    Apply {
        /// Path to TOML config file (default: labels.toml)
        #[arg(default_value = "labels.toml")]
        file: String,

        /// Dry run - show what would be done without making changes
        #[arg(short = 'n', long)]
        dry_run: bool,

        /// Skip labels that already exist instead of failing
        #[arg(short = 's', long)]
        skip_existing: bool,
    },

    /// Manage label templates
    Template {
        #[command(subcommand)]
        subcommand: TemplateSubcommands,
    },

    /// Generate shell completions
    Completion {
        #[command(subcommand)]
        subcommand: CompletionSubcommands,
    },
}

#[derive(Subcommand)]
pub enum TemplateSubcommands {
    /// List available templates
    List,

    /// Show template content
    Show {
        /// Template name
        name: String,
    },

    /// Apply a template to the current repository
    Apply {
        /// Template name
        name: String,

        /// Dry run - show what would be done without making changes
        #[arg(short = 'n', long)]
        dry_run: bool,

        /// Skip labels that already exist instead of failing
        #[arg(short = 's', long)]
        skip_existing: bool,
    },
}

#[derive(Subcommand)]
pub enum CompletionSubcommands {
    /// Generate bash completions
    ///
    /// Install with:
    ///   biao completion bash | sudo tee /usr/local/etc/bash_completion.d/biao
    Bash,

    /// Generate zsh completions
    ///
    /// Install with:
    ///   biao completion zsh | tee /usr/local/share/zsh/site-functions/_biao
    Zsh,

    /// Generate fish completions
    ///
    /// Install with:
    ///   biao completion fish | tee ~/.config/fish/completions/biao.fish
    Fish,

    /// Generate elvish completions
    ///
    /// Install with:
    ///   biao completion elvish | tee ~/.config/elvish/rc.elv
    Elvish,
}

#[derive(Subcommand)]
pub enum AuthSubcommands {
    /// Login to GitHub
    Login,

    /// Logout from GitHub
    Logout,

    /// Show authentication status
    Status,
}

pub async fn execute(args: Args) -> Result<()> {
    // Auth, Template, and Completion commands don't need git repo
    if matches!(args.command, Commands::Auth { .. } | Commands::Template { .. } | Commands::Completion { .. }) {
        if let Commands::Auth { subcommand } = args.command {
            return cmd_auth(subcommand).await;
        }
        if let Commands::Template { subcommand } = args.command {
            return cmd_template(subcommand).await;
        }
        if let Commands::Completion { subcommand } = args.command {
            return cmd_completion(subcommand).await;
        }
    }

    // Auto-detect git repository
    let _ = crate::git::find_git_root()?;
    let (owner, repo) = crate::git::get_repo_info()?;

    let client = GithubClient::new(owner, repo);

    match args.command {
        Commands::Auth { subcommand } => cmd_auth(subcommand).await?,
        Commands::Template { subcommand } => cmd_template(subcommand).await?,
        Commands::Completion { subcommand } => cmd_completion(subcommand).await?,
        Commands::List => cmd_list(&client).await?,
        Commands::Get { name } => cmd_get(&client, &name).await?,
        Commands::Create {
            name,
            color,
            description,
        } => cmd_create(&client, &name, &color, description).await?,
        Commands::Update {
            name,
            new_name,
            color,
            description,
        } => cmd_update(&client, &name, new_name, color, description).await?,
        Commands::Delete { name, force } => cmd_delete(&client, &name, force).await?,
        Commands::Apply { file, dry_run, skip_existing } => cmd_apply(&client, &file, dry_run, skip_existing).await?,
    }

    Ok(())
}

async fn cmd_auth(subcommand: Option<AuthSubcommands>) -> Result<()> {
    use std::process::Command;

    let subcommand = subcommand.unwrap_or(AuthSubcommands::Login);

    let gh_subcommand = match subcommand {
        AuthSubcommands::Login => "login",
        AuthSubcommands::Logout => "logout",
        AuthSubcommands::Status => "status",
    };

    let mut cmd = Command::new("gh");
    cmd.args(["auth", gh_subcommand]);

    let status = cmd.status().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            crate::error::BiaoError::GhNotFound {
                message: "github.com/cli/cli".to_string(),
            }
        } else {
            crate::error::BiaoError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to run gh auth {}: {}", gh_subcommand, e),
            ))
        }
    })?;

    if !status.success() {
        return Err(crate::error::BiaoError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("gh auth {} failed", gh_subcommand),
        )));
    }

    Ok(())
}

async fn cmd_list(client: &GithubClient) -> Result<()> {
    let labels = client.list_labels().await?;

    if labels.is_empty() {
        println!("Repository: {}", client.repo_url().cyan());
        println!("No labels found.");
        return Ok(());
    }

    println!("\nRepository: {}", client.repo_url().cyan());
    println!("{} Labels found:\n", labels.len());
    for label in labels {
        print_label(&label);
    }
    Ok(())
}

async fn cmd_get(client: &GithubClient, name: &str) -> Result<()> {
    println!("Repository: {}", client.repo_url().cyan());
    let label = client.get_label(name).await?;
    println!();
    print_label(&label);
    Ok(())
}

async fn cmd_create(
    client: &GithubClient,
    name: &str,
    color: &str,
    description: Option<String>,
) -> Result<()> {
    let color = normalize_color(color)?;

    let request = CreateLabelRequest {
        name: name.to_string(),
        color,
        description,
    };

    println!("Repository: {}", client.repo_url().cyan());
    let label = client.create_label(&request).await?;
    println!("\n✓ {} created successfully", "Label".green());
    print_label(&label);
    Ok(())
}

async fn cmd_update(
    client: &GithubClient,
    name: &str,
    new_name: Option<String>,
    color: Option<String>,
    description: Option<String>,
) -> Result<()> {
    let color = color.map(|c| normalize_color(&c)).transpose()?;

    let request = UpdateLabelRequest {
        name: new_name,
        color,
        description,
    };

    println!("Repository: {}", client.repo_url().cyan());
    let label = client.update_label(name, &request).await?;
    println!("\n✓ {} updated successfully", "Label".green());
    print_label(&label);
    Ok(())
}

async fn cmd_delete(client: &GithubClient, name: &str, force: bool) -> Result<()> {
    if !force {
        print!("Are you sure you want to delete '{}' from {}? [y/N]: ", name, client.repo_url().cyan());
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    client.delete_label(name).await?;
    println!("✓ {} '{}' from {} deleted", "Label".red(), name, client.repo_url().cyan());
    Ok(())
}

fn normalize_color(color: &str) -> Result<String> {
    let color = color.trim_start_matches('#');

    if color.len() != 6 {
        return Err(crate::error::BiaoError::InvalidInput(
            "Color must be 6 hex digits (e.g., ff0000)".to_string(),
        ));
    }

    // Validate hex
    if !color.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(crate::error::BiaoError::InvalidInput(
            "Invalid hex color format".to_string(),
        ));
    }

    Ok(color.to_lowercase())
}

async fn cmd_apply(client: &GithubClient, file: &str, dry_run: bool, skip_existing: bool) -> Result<()> {
    use crate::config::LabelConfig;
    use crate::models::{CreateLabelRequest, UpdateLabelRequest};

    println!("Repository: {}", client.repo_url().cyan());
    println!("Reading config from: {}\n", file.cyan());

    let config = LabelConfig::from_file(file)?;

    if !config.has_actions() {
        println!("No actions to perform. Config file is empty.");
        return Ok(());
    }

    if dry_run {
        println!("{}", "=== DRY RUN MODE ===".yellow().bold());
        println!("No changes will be made.\n");
    }

    let mut success_count = 0;
    let mut error_count = 0;
    let mut skipped_count = 0;

    // Process labels (create or update)
    if !config.labels.is_empty() {
        println!("{} Processing {} label(s):", "▶".green(), config.labels.len());
        for label in &config.labels {
            // First, handle update_if_match: rename matching labels to the new name
            if !label.update_if_match.is_empty() {
                let mut found_any = false;
                for old_name in &label.update_if_match {
                    print!("  {} Renaming '{}' → '{}'... ", "↻".blue(), old_name.cyan(), label.name.cyan());
                    
                    if dry_run {
                        println!("{}", "[DRY RUN]".yellow());
                        success_count += 1;
                        found_any = true;
                    } else {
                        let color = label.color.as_ref().map(|c| normalize_color(c)).transpose()?;
                        let request = UpdateLabelRequest {
                            name: Some(label.name.clone()),
                            color,
                            description: label.description.clone(),
                        };

                        match client.update_label(old_name, &request).await {
                            Ok(_) => {
                                println!("{}", "OK".green());
                                success_count += 1;
                                found_any = true;
                            }
                            Err(e) => {
                                let err_msg = format!("{}", e);
                                if err_msg.contains("Not Found") || err_msg.contains("404") {
                                    println!("{}", "NOT FOUND".yellow());
                                } else {
                                    println!("{}: {}", "FAILED".red(), e);
                                    error_count += 1;
                                }
                            }
                        }
                    }
                }
                
                // If none of the update_if_match labels were found, create a new label
                if !found_any && label.color.is_some() {
                    print!("  {} Creating '{}'... ", "✓".green(), label.name.cyan());
                    
                    if dry_run {
                        println!("{}", "[DRY RUN]".yellow());
                        success_count += 1;
                    } else {
                        let color = normalize_color(label.color.as_ref().unwrap())?;
                        let request = CreateLabelRequest {
                            name: label.name.clone(),
                            color,
                            description: label.description.clone(),
                        };

                        match client.create_label(&request).await {
                            Ok(_) => {
                                println!("{}", "OK".green());
                                success_count += 1;
                            }
                            Err(e) => {
                                println!("{}: {}", "FAILED".red(), e);
                                error_count += 1;
                            }
                        }
                    }
                }
                continue;
            }

            // If color is present, try to create (or update if exists)
            if let Some(color) = &label.color {
                print!("  {} Creating '{}'... ", "✓".green(), label.name.cyan());
                
                if dry_run {
                    println!("{}", "[DRY RUN]".yellow());
                    success_count += 1;
                } else {
                    let color = normalize_color(color)?;
                    let request = CreateLabelRequest {
                        name: label.name.clone(),
                        color,
                        description: label.description.clone(),
                    };

                    match client.create_label(&request).await {
                        Ok(_) => {
                            println!("{}", "OK".green());
                            success_count += 1;
                        }
                        Err(e) => {
                            // Check if it's a "already exists" error (422 status)
                            let err_msg = format!("{}", e);
                            let should_skip = skip_existing || label.skip_if_exists;
                            let should_update = label.update_if_exists;
                            
                            if err_msg.contains("already_exists") {
                                if should_update {
                                    // Try to update instead
                                    print!("{} (updating)... ", "EXISTS".yellow());
                                    let update_color = normalize_color(label.color.as_ref().unwrap())?;
                                    let update_request = UpdateLabelRequest {
                                        name: None,
                                        color: Some(update_color),
                                        description: label.description.clone(),
                                    };
                                    match client.update_label(&label.name, &update_request).await {
                                        Ok(_) => {
                                            println!("{}", "UPDATED".green());
                                            success_count += 1;
                                        }
                                        Err(update_err) => {
                                            println!("{}: {}", "FAILED".red(), update_err);
                                            error_count += 1;
                                        }
                                    }
                                } else if should_skip {
                                    println!("{}", "SKIPPED (already exists)".yellow());
                                    skipped_count += 1;
                                } else {
                                    println!("{}: {}", "FAILED".red(), e);
                                    error_count += 1;
                                }
                            } else {
                                println!("{}: {}", "FAILED".red(), e);
                                error_count += 1;
                            }
                        }
                    }
                }
            } else {
                // No color means update only
                print!("  {} Updating '{}'... ", "✓".blue(), label.name.cyan());
                
                if dry_run {
                    println!("{}", "[DRY RUN]".yellow());
                    success_count += 1;
                } else {
                    let request = UpdateLabelRequest {
                        name: None,
                        color: None,
                        description: label.description.clone(),
                    };

                    match client.update_label(&label.name, &request).await {
                        Ok(_) => {
                            println!("{}", "OK".green());
                            success_count += 1;
                        }
                        Err(e) => {
                            println!("{}: {}", "FAILED".red(), e);
                            error_count += 1;
                        }
                    }
                }
            }
        }
        println!();
    }

    // Process deletes
    if !config.delete.is_empty() {
        println!("{} Deleting {} label(s):", "▶".red(), config.delete.len());
        for name in &config.delete {
            print!("  {} Deleting '{}'... ", "✗".red(), name.cyan());
            
            if dry_run {
                println!("{}", "[DRY RUN]".yellow());
                success_count += 1;
            } else {
                match client.delete_label(name).await {
                    Ok(_) => {
                        println!("{}", "OK".green());
                        success_count += 1;
                    }
                    Err(e) => {
                        println!("{}: {}", "FAILED".red(), e);
                        error_count += 1;
                    }
                }
            }
        }
        println!();
    }

    // Summary
    println!("{}", "=== Summary ===".bold());
    println!("  {} {}", "Success:".green(), success_count);
    if skipped_count > 0 {
        println!("  {} {}", "Skipped:".yellow(), skipped_count);
    }
    if error_count > 0 {
        println!("  {} {}", "Failed:".red(), error_count);
    }

    if dry_run {
        println!("\n{}", "This was a dry run. No actual changes were made.".yellow());
    }

    Ok(())
}

async fn cmd_completion(subcommand: CompletionSubcommands) -> Result<()> {
    use clap::CommandFactory;

    let mut cmd = Args::command();

    match subcommand {
        CompletionSubcommands::Bash => {
            use clap_complete::shells::Bash;
            clap_complete::generate(Bash, &mut cmd, "biao", &mut std::io::stdout());
        }
        CompletionSubcommands::Zsh => {
            use clap_complete::shells::Zsh;
            clap_complete::generate(Zsh, &mut cmd, "biao", &mut std::io::stdout());
        }
        CompletionSubcommands::Fish => {
            use clap_complete::shells::Fish;
            clap_complete::generate(Fish, &mut cmd, "biao", &mut std::io::stdout());
        }
        CompletionSubcommands::Elvish => {
            use clap_complete::shells::Elvish;
            clap_complete::generate(Elvish, &mut cmd, "biao", &mut std::io::stdout());
        }
    }

    Ok(())
}

fn print_label(label: &crate::models::GithubLabel) {
    let color_display = format!("■");
    let color_rgb = format!("#{}", label.color);

    println!("  Name:        {}", label.name.cyan());
    println!("  Color:       {} {}", color_display, color_rgb);
    if let Some(desc) = &label.description {
        println!("  Description: {}", desc);
    }
    println!("  URL:         {}", label.url.dimmed());
    println!();
}

async fn cmd_template(subcommand: TemplateSubcommands) -> Result<()> {
    use crate::templates::TemplateManager;

    let manager = TemplateManager::new()?;

    match subcommand {
        TemplateSubcommands::List => {
            let templates = manager.list()?;
            println!("{}", "Available Templates:".bold());
            println!();

            if templates.is_empty() {
                println!("No templates found.");
                return Ok(());
            }

            for template in templates {
                let path_display = if template.path.as_os_str() == "__builtin__" {
                    "(built-in)".to_string()
                } else {
                    template.path.display().to_string()
                };

                println!(
                    "  {} - {}  [{}]",
                    template.name.cyan().bold(),
                    template.description,
                    path_display
                );
            }
            println!();
            println!("Use {} to apply a template", "biao template apply <name>".italic());
        }

        TemplateSubcommands::Show { name } => {
            let content = manager.get(&name)?;
            println!("{}", "Template: ".bold());
            println!("{}\n", name.cyan().bold());
            println!("{}", content);
        }

        TemplateSubcommands::Apply {
            name,
            dry_run,
            skip_existing,
        } => {
            let content = manager.get(&name)?;
            println!("Repository: {}", "auto-detected".cyan());
            println!("Template: {}\n", name.cyan());

            // We need to get the client for this
            // Since we're here, we know the git repo was already validated
            let _ = crate::git::find_git_root()?;
            let (owner, repo) = crate::git::get_repo_info()?;
            let client = GithubClient::new(owner, repo);

            // Write template to temp file
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let temp_file = format!("/tmp/biao-template-{}.toml", timestamp);
            std::fs::write(&temp_file, &content).map_err(|e| {
                crate::error::BiaoError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to write template to temp file: {}", e),
                ))
            })?;

            // Apply the temp file
            cmd_apply(&client, &temp_file, dry_run, skip_existing).await?;

            // Clean up
            let _ = std::fs::remove_file(&temp_file);
        }
    }

    Ok(())
}
