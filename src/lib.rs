#![allow(unused_variables)]

use git2::{Oid, Repository};

// -> Open & Select Your Repository
pub fn open_repo() -> Repository {
    let path = std::env::args().nth(1).unwrap_or(".".to_string());
    Repository::open(path.as_str()).expect("Couldn't open repo")
}

// -> Initial Head & Target
pub fn initial_head() -> Oid {
    let repo = open_repo();
    let initial_head = repo.head().unwrap();
    initial_head.target().unwrap()
}

// -> View all fullcommit log/detail
pub fn get_fullcommit_log() {
    let repo = open_repo();
    let commit = repo.find_commit(initial_head()).unwrap();
    
    let timestamp = commit.time().seconds();
    let tm = time::at(time::Timespec::new(timestamp, 0));

    let commit_message = commit.message();
    let commit_message_result = commit_message.as_deref().unwrap_or("Default String");
    
    let commit_id = commit.id();

    let commit_author = commit.author();
    let commit_author_name = commit_author.name();
    let commit_author_email = commit_author.email();
    let commit_committer_name = commit.committer();


    println!(
        "Commit Message Log: \n {:#?}",
        commit_message_result 
    )
}
