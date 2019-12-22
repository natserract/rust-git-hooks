#![allow(unused_variables)]

use std::thread::sleep;
use std::time::Duration;
use futures::future::join4;

use git2::{Oid, Repository, BranchType};

// -> Open & Select Your Repository
fn open_repo() -> Repository {
    let path = std::env::args().nth(1).unwrap_or(".".to_string());
    Repository::open(path.as_str()).expect("Couldn't open repo")
}

// -> Initial Head & Target
fn initial_head() -> Oid {
    let repo = open_repo();
    let initial_head = repo.head().unwrap();
    initial_head.target().unwrap()
}

// -> Convert from String to str
fn convert_to_str(target: Option<&str>) -> &str {
    target.unwrap_or("add")
}

// -> Commit Logging
async fn get_fullcommit_log() {
    let timeout = Duration::from_secs(1);
    sleep(timeout);

    let repo = open_repo();
    let commit = repo.find_commit(initial_head()).unwrap();
    
    let timestamp = commit.time().seconds();
    let tm = time::at(time::Timespec::new(timestamp, 0));
    let time = tm.rfc822();

    let commit_message = convert_to_str(commit.message());
    let commit_message_bytes = commit.message_bytes();
    let commit_raw_header = convert_to_str(commit.raw_header());
    let commit_summary = commit.summary();

    let commit_id = commit.id();

    let commit_author = commit.author();
    let commit_author_name = convert_to_str(commit_author.name());
    let commit_author_email = convert_to_str(commit_author.email());

    let commiter = commit.committer();
    let commiter_name = convert_to_str(commiter.name());
    let commiter_email = convert_to_str(commiter.email());


    println!(
        "\n Last Commit:\n Id -> {} \n Message -> {} Committer Name -> {} \n Committer Email -> {} \n Time -> {} \n Raw -> {:#?} \n",
        commit_id, 
        commit_message, 
        commiter_name,
        commiter_email,
        time,
        commit_raw_header,
    )
}

async fn create_new_branch() {
    let timeout = Duration::from_secs(3);
    sleep(timeout);

    let repo = open_repo();
    let commit = repo.find_commit(initial_head()).unwrap();
    let mut branch = repo.branch("test_branch", &commit, true).unwrap();

    // -> sets the default remote branch
    branch.set_upstream(Some("master")).unwrap();
    branch.upstream().unwrap();

    println!("Branch has been succesfully created! If you wanna check type command git branch -a");
}


async fn rename_branch() {
    let timeout = Duration::from_secs(5);
    sleep(timeout);

    let repo = open_repo();
    let commit = repo.find_commit(initial_head()).unwrap();
    let mut find_branch = repo.find_branch("test_branch",BranchType::Local).unwrap();
    let mut renamed_branch = find_branch.rename("test_branch_1", true).unwrap();

    // -> sets the default remote branch
    renamed_branch.set_upstream(Some("master")).unwrap();
    renamed_branch.upstream().unwrap();

    println!("Branch has been succesfully renamed! If you wanna check type command git branch -a");
}

async fn delete_branch() {
    let timeout = Duration::from_secs(7);
    sleep(timeout);

    let repo = open_repo();
    let commit = repo.find_commit(initial_head()).unwrap();
    let mut find_branch = repo.find_branch("test_branch_1",BranchType::Local).unwrap();

    find_branch.delete().unwrap();

    println!("Branch has been succesfully deleted! If you wanna check type command git branch -a");
}


pub async fn merge_all_hooks() {
    let commit_log = get_fullcommit_log();
    let new_branch = create_new_branch();
    let rename_branch = rename_branch();
    let delete_branch = delete_branch();
  
    let race_all = join4(commit_log, new_branch, rename_branch, delete_branch);
    race_all.await;
    
}