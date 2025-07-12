use std::{error, fs, io::{self, Write}, process::exit};
use std::path::PathBuf;
use chrono::{Duration, Local};
use colored::Colorize;
use keyring::Entry;
use rpassword::read_password;
use serde_json::json;
use crate::{cli::{Cli, Commands, NewProject, Priority}, utils::{api::post_login, auth::{jwt_expired, refresh_access_token}, display_format::log_err}};
use crate::utils::api::{delete_task, get_projects, get_tasks, get_uniq_proj, get_user, post_project, post_task, put_task};
use crate::utils::config_path::{get_config_path, load_config, Config};
use crate::utils::display_format::{format_pretty_date, natural_to_datetime, print_project, print_task};
use crate::utils::types::{PriorityType, Project, Task, User};
use crate::utils::auth::AuthStore;



// HANDLER: Use project
async fn handle_use (pid: u32, auth_store: &AuthStore) -> Result<(), Box<dyn error::Error>> {
    if let Some(mut config) = load_config() {
        // Get token from keyring
        let token = match auth_store.access.get_password() {
            Ok(t) => t,
            Err(_) => {
                println!("\n{} You may not be logged in. Run {}", " ".red(), "cues login".yellow());
                exit(0);
            }
        };

        // Check if token is expired, if yes then refresh and get a new token
        if jwt_expired(config.expires_at.as_str()) {
            let refresh_token = match auth_store.refresh.get_password() {
                Ok(t) => t,
                Err(_) => {
                    println!("\n{} Refresh token couldn't be found. Log in again by running {}", " ".red(), "cues login".yellow());
                    exit(0);
                }
            };

            let new_token = refresh_access_token(&refresh_token).await?;
            auth_store.access.set_password(&new_token);
            let token = new_token;
        }

        // API call
        let res = get_uniq_proj(token.as_str(), &pid).await?;

        // Extracting value from response
        if let Some(projval) = res.get("project") {
            let proj: Project = serde_json::from_value(projval.clone())?;

            // Setting config file properties
            config.current_project_id = pid.clone();
            config.current_project = proj.name.clone();

            // Writing new properties to file
            let config_path = get_config_path().expect("Could not determine config directory");
            let json = serde_json::to_string_pretty(&config)?;
            fs::write(&config_path, json)?;

            println!("\n{}\n{}", "  Set active project:".green(), proj.name.clone());
        } else {
            println!();
            log_err(res);
        }
    } else {
        println!("\n{} Config file missing. Run {} to log in to your account.", " ".red(), "cues login".yellow());
    }

    Ok(())
}

// HANDLER: Get current active/working project
async fn handle_cwp (auth_store: &AuthStore) -> Result<(), Box<dyn error::Error>> {
    if let Some(config) = load_config() {
        if (config.current_project_id == 0) {
            // If current_project_id == 0, it means no active project is set. So, logging this info
            // to user in order to tell them what to be done.
            println!("\n{} {} {} {} {} {}", " ".bold().red(), "You have not set any project as active. Log in using", "cues login".yellow(), "and run", "cues use".yellow(), "to set an active project.");
        } else {
            // Else, get the current project name and log it as CWP (Current Working Project)
            let cwp = config.current_project;
            let pid = format!("[{}]", config.current_project_id).yellow();
            println!("\n{}\n{} {}", "Active Project:".yellow(), pid, cwp);
        }
    } else {
        println!("\n{} Config file missing. Run {} to log in to your account.", " ".red(), "cues login".yellow());
    }

    Ok(())
}

// HANDLER: Create new project
async fn handle_new_project (name: String, auth_store: &AuthStore) -> Result<(), Box<dyn error::Error>> {
    if let Some(config) = load_config() {
        // Get token from keyring
        let token = match auth_store.access.get_password() {
            Ok(t) => t,
            Err(_) => {
                println!("\n{} You may not be logged in. Run {}", " ".red(), "cues login".yellow());
                exit(0);
            }
        };

        // Check if token is expired, if yes then refresh and get a new token
        if jwt_expired(config.expires_at.as_str()) {
            let refresh_token = match auth_store.refresh.get_password() {
                Ok(t) => t,
                Err(_) => {
                    println!("\n{} Refresh token couldn't be found. Log in again by running {}", " ".red(), "cues login".yellow());
                    exit(0);
                }
            };

            let new_token = refresh_access_token(&refresh_token).await?;
            auth_store.access.set_password(&new_token);
            let token = new_token;
        }

        let payload = json!({
            "name": name.clone()
        });

        // API call
        let res = post_project(token.as_str(), &payload).await?;

        // Extracting data from response
        if let Some(projval) = res.get("project") {
            let project: Project = serde_json::from_value(projval.clone())?;

            let pname = &project.name;
            let pid = project.id;

            println!("\n{} The following project was added:\n", " ".green());
            print_project(project);
        } else {
            println!();
            log_err(res);
        }
    } else {
        println!("\n{} You need to log in first to run this command. Run {} to log in to your account.", " ".red(), "cues login".yellow());
    }

    Ok(())
}

// HANDLER: List all projects
async fn handle_list_projects (auth_store: &AuthStore) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(config) = load_config() {
        // Get token from keyring
        let token = match auth_store.access.get_password() {
            Ok(t) => t,
            Err(_) => {
                println!("\n{} You may not be logged in. Run {}", " ".red(), "cues login".yellow());
                exit(0);
            }
        };

        // Check if token is expired, if yes then refresh and get a new token
        if jwt_expired(config.expires_at.as_str()) {
            let refresh_token = match auth_store.refresh.get_password() {
                Ok(t) => t,
                Err(_) => {
                    println!("\n{} Refresh token couldn't be found. Log in again by running {}", " ".red(), "cues login".yellow());
                    exit(0);
                }
            };

            let new_token = refresh_access_token(&refresh_token).await?;
            auth_store.access.set_password(&new_token);
            let token = new_token;
        }

        // API call
        let res = get_projects(token.as_str()).await?;

        // Extracting data from response
        if let Some(projects_val) = res.get("projects") {
            let projects: Vec<Project> = serde_json::from_value(projects_val.clone())?;

            // Pretty-printing all projects
            println!("\nProjects:\n");
            for proj in projects {
                print_project(proj);
            }
        } else {
            println!();
            log_err(res);
        }
    } else {
        println!("\n{} You need to log in first to run this command. Run {} to log in to your account.", " ".red(), "cues login".yellow());
    }

    Ok(())
}

// HANDLER: Add new task
async fn handle_task_add (title: String, desc: Option<String>, due: Option<String>, priority: Option<PriorityType>, auth_store: &AuthStore) -> Result<(), Box<dyn error::Error>> {
    // Parse custom format ("today 16:00" or "friday 4:00" etc) into proper datetime format
    let parsed_due = if let Some(due_date) = due {
        match natural_to_datetime(&due_date) {
            Some(due_str) => Some(due_str),
            None => {
                eprintln!("{} Invalid due date format.", " ".red());
                return Ok(());
            }
        }
    } else {
        None
    };

    if let Some(config) = load_config() {
        // Get token from keyring
        let token = match auth_store.access.get_password() {
            Ok(t) => t,
            Err(_) => {
                println!("\n{} You may not be logged in. Run {}", " ".red(), "cues login".yellow());
                exit(0);
            }
        };

        // Check if token is expired, if yes then refresh and get a new token
        if jwt_expired(config.expires_at.as_str()) {
            let refresh_token = match auth_store.refresh.get_password() {
                Ok(t) => t,
                Err(_) => {
                    println!("\n{} Refresh token couldn't be found. Log in again by running {}", " ".red(), "cues login".yellow());
                    exit(0);
                }
            };

            let new_token = refresh_access_token(&refresh_token).await?;
            auth_store.access.set_password(&new_token);
            let token = new_token;
        }

        // Building payload one-by-one based on data provided by user
        let mut payload = json!({
            "title": title,
            "projectId": config.current_project_id,
        });

        if let Some(d) = desc {
            payload["description"] = json!(d);
        }

        if let Some(due_str) = parsed_due {
            payload["due"] = json!(due_str);
        }

        if let Some(p) = priority {
            payload["priority"] = json!(format!("{:?}", p));
        }

        // API call
        let res = post_task(token.as_str(), &payload).await?;

        // Extracting data from response
        if let Some(taskval) = res.get("task") {
            let task: Task = serde_json::from_value(taskval.clone())?;

            println!("\n{} The following task was added:\n", " ".green());
            print_task(&task, false);  // Pretty-printing task
        } else {
            println!();
            log_err(res);
        }
    } else {
        println!("\n{} You need to log in first to run this command. Run {} to log in to your account.", " ".red(), "cues login".yellow());
    }

    Ok(())
}

// HANDLER: List tasks (in cwp)
async fn handle_list_tasks (all: bool, auth_store: &AuthStore) -> Result<(), Box<dyn error::Error>> {
    if let Some(config) = load_config() {
        // Get token from keyring
        let token = match auth_store.access.get_password() {
            Ok(t) => t,
            Err(_) => {
                println!("\n{} You may not be logged in. Run {}", " ".red(), "cues login".yellow());
                exit(0);
            }
        };

        // Check if token is expired, if yes then refresh and get a new token
        if jwt_expired(config.expires_at.as_str()) {
            let refresh_token = match auth_store.refresh.get_password() {
                Ok(t) => t,
                Err(_) => {
                    println!("\n{} Refresh token couldn't be found. Log in again by running {}", " ".red(), "cues login".yellow());
                    exit(0);
                }
            };

            let new_token = refresh_access_token(&refresh_token).await?;
            auth_store.access.set_password(&new_token);
            let token = new_token;
        }

        let pid = config.current_project_id;  // Get current project id from config

        // For 'cues tasks --all'
        if all {
            // Concurrently make two API calls
            let (task_res, proj_res) = tokio::join!(
                get_tasks(token.as_str()),      // API call (Get tasks)
                get_projects(token.as_str()),   // API call (Get projects)
            );

            let task_res = task_res?;
            let proj_res = proj_res?;

            // Extracting data from both responses

            let Some(tv) = task_res.get("tasks") else {
                println!();
                log_err(task_res);
                return Ok(());
            };

            let Some(pv) = proj_res.get("projects") else {
                println!();
                log_err(proj_res);
                return Ok(());
            };

            let tasks: Vec<Task> = serde_json::from_value(tv.clone())?;

            if tasks.is_empty() {
                // If no tasks are present, tell the user instead of printing an empty task list
                println!("\n{} {} {} {} {} {}", "  ".bold(), "No tasks have been created. Run".yellow(), "cues add".blue(), "to add new tasks, or".yellow(), "cues new project".blue(), "to create a new project.");
            } else {
                // Else, print the tasks grouped by project
                let projects: Vec<Project> = serde_json::from_value(pv.clone())?;

                if !projects.is_empty() {
                    println!("\n{} Available tasks:\n", " ".green());

                    for project in projects.iter() {
                        // Get tasks under this project member
                        let proj_tasks: Vec<&Task> = tasks.iter().filter(|t| t.project_id == project.id).collect();

                        // If no tasks in this project, skip to next project
                        if proj_tasks.is_empty() {
                            continue;
                        }

                        // Else, print the project name and then list the tasks (all pretty-printed)

                        println!("{} {}\n", " ".bold().yellow(), project.name.bold().yellow());

                        for task in proj_tasks {
                            print_task(task, false);
                        }

                        println!();
                    }
                } else {
                    println!("\n{} {} {} {}", "  ".bold(), "No projects are defined. Run".yellow(), "cues new project".blue(), "to add a new project.".yellow());
                }
            }
            
        } else {
            // For 'cues tasks' (No --all or -a flag)

            // API call (get tasks)
            // [No concurrency here as just one API call is made]
            let task_res = get_tasks(token.as_str()).await?;

            // Extracting data from response
            let Some(tv) = task_res.get("tasks") else {
                println!();
                log_err(task_res);
                return Ok(());
            };

            let tasks: Vec<Task> = serde_json::from_value(tv.clone())?;

            if tasks.is_empty() {
                // If no tasks are present, tell the user instead of printing an empty list
                println!("\n{} {} {} {} {} {}", "  ".bold(), "No tasks present in the current project. Run".yellow(), "cues add".blue(), "to add new tasks, or".yellow(), "cues use".blue(), "to use a different project.".yellow());
            } else {
                // Else, pretty-print all the tasks in the CWP
                println!("\n{} Available tasks:\n", " ".green());

                for task in tasks.iter().filter(|t| t.project_id == pid) {
                    print_task(task, false);
                }
            }
        }
    } else {
        println!("\n{} You need to log in first to run this command. Run {} to log in to your account.", " ".red(), "cues login".yellow());
    }

    Ok(())
}

// HANDLER: Mark task as done
async fn handle_task_done (id: u32, auth_store: &AuthStore) -> Result<(), Box<dyn error::Error>> {
    if let Some(config) = load_config() {
        // Get token from keyring
        let token = match auth_store.access.get_password() {
            Ok(t) => t,
            Err(_) => {
                println!("\n{} You may not be logged in. Run {}", " ".red(), "cues login".yellow());
                exit(0);
            }
        };

        // Check if token is expired, if yes then refresh and get a new token
        if jwt_expired(config.expires_at.as_str()) {
            let refresh_token = match auth_store.refresh.get_password() {
                Ok(t) => t,
                Err(_) => {
                    println!("\n{} Refresh token couldn't be found. Log in again by running {}", " ".red(), "cues login".yellow());
                    exit(0);
                }
            };

            let new_token = refresh_access_token(&refresh_token).await?;
            auth_store.access.set_password(&new_token);
            let token = new_token;
        }

        let payload = json!({
            "isDone": true,
        });

        // API call
        let res = put_task(token.as_str(), &id, &payload).await?;

        // Extracting data from response
        if let Some(taskval) = res.get("task") {
            let task: Task = serde_json::from_value(taskval.clone())?;

            println!("\n{}", " Marked following task as done:\n".green());
            print_task(&task, false);  // Pretty-print task marked as Done

            println!("\nRun {} to view all tasks in current project.", "cues tasks".yellow());
        } else {
            println!();
            log_err(res);
        }
    } else {
        println!("\n{} You need to log in first to run this command. Run {} to log in to your account.", " ".red(), "cues login".yellow());
    }

    Ok(())
}

// HANDLER: Edit task
async fn handle_task_edit (id: u32, title: Option<String>, desc: Option<String>, priority: Option<PriorityType>, due: Option<String>, done: Option<bool>, auth_store: &AuthStore) -> Result<(), Box<dyn error::Error>> {
    if let Some(config) = load_config() {
        // Get token from keyring
        let token = match auth_store.access.get_password() {
            Ok(t) => t,
            Err(_) => {
                println!("\n{} You may not be logged in. Run {}", " ".red(), "cues login".yellow());
                exit(0);
            }
        };

        // Check if token is expired, if yes then refresh and get a new token
        if jwt_expired(config.expires_at.as_str()) {
            let refresh_token = match auth_store.refresh.get_password() {
                Ok(t) => t,
                Err(_) => {
                    println!("\n{} Refresh token couldn't be found. Log in again by running {}", " ".red(), "cues login".yellow());
                    exit(0);
                }
            };

            let new_token = refresh_access_token(&refresh_token).await?;
            auth_store.access.set_password(&new_token);
            let token = new_token;
        }

        // Parse given format ("today 16:00" or "friday 4:00" etc) to a valid datetime format
        let parsed_due = if let Some(due_date) = due {
            match natural_to_datetime(&due_date) {
                Some(due_str) => Some(due_str),
                None => {
                    eprintln!("{} Invalid due date format.", " ".red());
                    return Ok(());
                }
            }
        } else {
            None
        };

        // Build the payload one-by-one based on data provided by user

        let mut payload = json!({});

        if let Some(t) = title {
            payload["title"] = json!(t);
        }

        if let Some(d) = desc {
            payload["description"] = json!(d);
        }

        if let Some(p) = priority {
            payload["priority"] = json!(format!("{:?}", p));
        }

        if let Some(due_str) = parsed_due {
            payload["due"] = json!(due_str);
        }

        if let Some(done_status) = done {
            payload["isDone"] = json!(done_status);
        }

        // API call
        let res = put_task(token.as_str(), &id, &payload).await?;

        // Extracting data from response
        if let Some(taskval) = res.get("task") {
            let task: Task = serde_json::from_value(taskval.clone())?;

            println!("\n{}", " Following task has been updated:\n".green());
            print_task(&task, false);  // Pretty-print the edited task

            println!("\nRun {} to view all tasks in current project.", "cues tasks".yellow());
        } else {
            println!();
            log_err(res);
        }
    } else {
        println!("\n{} You need to log in first to run this command. Run {} to log in to your account.", " ".red(), "cues login".yellow());
    }

    Ok(())
}

// HANDLER: Delete task
async fn handle_task_delete (id: u32, auth_store: &AuthStore) -> Result<(), Box<dyn error::Error>> {
    if let Some(config) = load_config() {
        // Get token from keyring
        let token = match auth_store.access.get_password() {
            Ok(t) => t,
            Err(_) => {
                println!("\n{} You may not be logged in. Run {}", " ".red(), "cues login".yellow());
                exit(0);
            }
        };

        // Check if token is expired, if yes then refresh and get a new token
        if jwt_expired(config.expires_at.as_str()) {
            let refresh_token = match auth_store.refresh.get_password() {
                Ok(t) => t,
                Err(_) => {
                    println!("\n{} Refresh token couldn't be found. Log in again by running {}", " ".red(), "cues login".yellow());
                    exit(0);
                }
            };

            let new_token = refresh_access_token(&refresh_token).await?;
            auth_store.access.set_password(&new_token);
            let token = new_token;
        }

        // API call
        let res = delete_task(token.as_str(), &id).await?;

        // Extracting data from response
        if let Some(taskval) = res.get("task") {
            let task: Task = serde_json::from_value(taskval.clone())?;

            println!("\n{}", " Following task has been deleted:\n".green());
            print_task(&task, false);  // Pretty-print the deleted task

            println!("\nRun {} to view all available tasks in current project.", "cues tasks".yellow());
        } else {
            println!();
            log_err(res);
        }
    } else {
        println!("\n{} You need to log in first to run this command. Run {} to log in to your account.", " ".red(), "cues login".yellow());
    }

    Ok(())
}

// HANDLER: Get user details (whoami)
async fn whoami (auth_store: &AuthStore) -> Result<(), Box<dyn error::Error>> {
    if let Some(config) = load_config() {
        // Get token from keyring
        let token = match auth_store.access.get_password() {
            Ok(t) => t,
            Err(_) => {
                println!("\n{} You may not be logged in. Run {}", " ".red(), "cues login".yellow());
                exit(0);
            }
        };

        // Check if token is expired, if yes then refresh and get a new token
        if jwt_expired(config.expires_at.as_str()) {
            let refresh_token = match auth_store.refresh.get_password() {
                Ok(t) => t,
                Err(_) => {
                    println!("\n{} Refresh token couldn't be found. Log in again by running {}", " ".red(), "cues login".yellow());
                    exit(0);
                }
            };

            let new_token = refresh_access_token(&refresh_token).await?;
            auth_store.access.set_password(&new_token);
            let token = new_token;
        }

        // API call
        let res = get_user(token.as_str()).await?;

        // Extracting data from response

        let Some(userval) = res.get("user") else {
            println!();
            log_err(res);
            return Ok(());
        };

        let user: User = serde_json::from_value(userval.clone())?;
        let user_joined_date = format_pretty_date(user.created_at.as_str())?;

        // Displaying user information

        let cues_ascii: &str = r#"
        

 ██████╗██╗   ██╗███████╗███████╗
██╔════╝██║   ██║██╔════╝██╔════╝
██║     ██║   ██║█████╗  ███████╗
██║     ██║   ██║██╔══╝  ╚════██║
╚██████╗╚██████╔╝███████╗███████║
 ╚═════╝ ╚═════╝ ╚══════╝╚══════╝
                                 
        "#;

        println!("{}", cues_ascii.yellow());
        println!("\n{}", "  User Information".yellow());
        println!("──────────────────────────────");
        println!("\n\n{} {}", "  Username:".blue(), user.username);
        println!("\n{} {}", "󰇮  Email address:".blue(), user.email);
        println!("\n{} {}", "  Joined on:".blue(), user_joined_date);
    } else {
        println!("\n{} You need to log in first to run this command. Run {} to log in to your account.", " ".red(), "cues login".yellow());
    }

    Ok(())
}

// HANDLER: Log in
async fn handle_login (auth_store: &AuthStore) -> Result<(), Box<dyn error::Error>> {
     let config_path = get_config_path().expect("Could not determine config directory");

     if let Some(parent) = config_path.parent() {
         fs::create_dir_all(parent)?;
     }

    let cues_ascii: &str = r#"
        

 ██████╗██╗   ██╗███████╗███████╗
██╔════╝██║   ██║██╔════╝██╔════╝
██║     ██║   ██║█████╗  ███████╗
██║     ██║   ██║██╔══╝  ╚════██║
╚██████╗╚██████╔╝███████╗███████║
 ╚═════╝ ╚═════╝ ╚══════╝╚══════╝
                                 

    "#;

    println!("{}", cues_ascii.yellow());

    println!("\n{}", "󰍂 Log in to Cues CLI".yellow());
    println!("──────────────────────────────");

    print!("\n\n{} {} ", " ".blue(), "Username or Email:".bold());
    io::stdout().flush()?;
    let mut username_or_email = String::new();
    io::stdin().read_line(&mut username_or_email)?;
    let username_or_email = username_or_email.trim();

    print!("\n{} {} ", " ".blue(), "Password:".bold());
    io::stdout().flush()?;
    let pswd = read_password()?.trim().to_string();

    let mut payload = json!({
        "password": pswd,
    });

    if username_or_email.contains("@") {
        payload["email"] = json!(username_or_email);
    } else {
        payload["username"] = json!(username_or_email);
    }

    let res = post_login(&payload).await?;

    let Some(tokenval) = res.get("accessToken").and_then(|val| val.as_str()) else {
        println!();
        log_err(res);
        return Ok(());
    };

    let Some(refreshval) = res.get("refreshToken").and_then(|val| val.as_str()) else {
        println!();
        log_err(res);
        return Ok(());
    };

    let access_token = tokenval.to_string();
    let refresh_token = refreshval.to_string();

    let expires_at = (Local::now() + Duration::hours(1)).to_rfc3339();

    auth_store.access.set_password(&access_token)?;
    auth_store.refresh.set_password(&refresh_token)?;


    let config = Config {
        expires_at: expires_at,
        current_project: String::new(),
        current_project_id: 0,
    };


    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, json)?;

    println!("\n\n{}{}{}.", "  ".green(), "Logged in succesfully, as ", username_or_email.yellow());

    Ok(())
}

pub async fn handle_logout (auth_store: &AuthStore) -> Result<(), Box<dyn error::Error>> {
    if let Some(mut config) = load_config() {
        let cues_ascii: &str = r#"
        

 ██████╗██╗   ██╗███████╗███████╗
██╔════╝██║   ██║██╔════╝██╔════╝
██║     ██║   ██║█████╗  ███████╗
██║     ██║   ██║██╔══╝  ╚════██║
╚██████╗╚██████╔╝███████╗███████║
 ╚═════╝ ╚═════╝ ╚══════╝╚══════╝
                                 

        "#;

        println!("{}", cues_ascii.yellow());

        match auth_store.access.delete_credential() {
            Ok(_) => println!(""),
            Err(e) => eprintln!("{} Failed to clear access token: {}", "".red(), e),
        }

        match auth_store.refresh.delete_credential() {
            Ok(_) => print!(""),
            Err(e) => eprintln!("{} Failed to clear refresh token: {}", "".red(), e),
        }

        config.expires_at = String::new();
        config.current_project = String::new();
        config.current_project_id = 0;

        let config_path = get_config_path().expect("Could not determine config directory");
        let json = serde_json::to_string_pretty(&config)?;

        fs::write(&config_path, json)?;

        println!("\n{} {} {}", " ".green(), "Logged out successfully. Log in using the command".green(), "cues login".yellow());
    }

    Ok(())
}

// Cues CLI Handler
pub async fn handle_cli (cmd: Cli, auth_store: AuthStore) -> Result<(), Box<dyn std::error::Error>> {
    match cmd.command {
        // PROJECT sub-commands
        Commands::Projects => {
            handle_list_projects(&auth_store).await?;
        },
        Commands::Cwp => {
            handle_cwp(&auth_store).await?;
        },
        Commands::Use { pid } => {
            handle_use(pid, &auth_store).await?;
        },
        Commands::New { kind: NewProject::Project { name } } => {
            handle_new_project(name, &auth_store).await?;
        },

        // TASK sub-commands
        Commands::Add { title, priority, desc, due } => {
            let priority_new: Option<PriorityType> = match priority {
                Some(Priority::Low) => Some(PriorityType::Low),
                Some(Priority::Medium) => Some(PriorityType::Medium),
                Some(Priority::High) => Some(PriorityType::High),
                None => None,
            };
            handle_task_add(title, desc, due, priority_new, &auth_store).await?;
        },
        Commands::Tasks { all } => {
            handle_list_tasks(all, &auth_store).await?;
        },
        Commands::Done { task_id } => {
            handle_task_done(task_id, &auth_store).await?;
        },
        Commands::Edit { task_id, title, priority, desc, due, done } => {
            let priority_new: Option<PriorityType> = match priority {
                Some(Priority::Low) => Some(PriorityType::Low),
                Some(Priority::Medium) => Some(PriorityType::Medium),
                Some(Priority::High) => Some(PriorityType::High),
                None => None,
            };

            handle_task_edit(task_id, title, desc, priority_new, due, done, &auth_store).await?;
        },
        Commands::Delete { task_id } => {
            handle_task_delete(task_id, &auth_store).await?;
        },
        Commands::Login => {
            handle_login(&auth_store).await?;
        },
        Commands::Whoami => {
            whoami(&auth_store).await?;
        },
        Commands::Logout => {
            handle_logout(&auth_store).await?;
        }
    }

    Ok(())
}
