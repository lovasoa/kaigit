#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use serde::Serialize;

#[derive(Serialize)]
enum ObjectContents {
  Commit {
    message: String,
    author: String,
    parents: Vec<String>,
    tree_id: String,
  },
  Blob {
    size: usize,
  },
  Tree {
    children: Vec<TreeEntry>,
  },
  Tag {
    name: String,
    target: String,
  },
  Unknown,
}

#[derive(Serialize)]
struct TreeEntry {
  name: String,
  id: String,
}

impl From<git2::Commit<'_>> for ObjectContents {
  fn from(commit: git2::Commit) -> Self {
    ObjectContents::Commit {
      message: commit.message().unwrap_or_default().to_string(),
      author: commit.author().name().unwrap_or_default().to_string(),
      parents: commit.parents().map(|p| p.id().to_string()).collect(),
      tree_id: commit.tree_id().to_string(),
    }
  }
}

impl From<git2::Tree<'_>> for ObjectContents {
  fn from(tree: git2::Tree) -> Self {
    ObjectContents::Tree {
      children: tree
        .into_iter()
        .map(|entry| TreeEntry {
          id: entry.id().to_string(),
          name: entry.name().unwrap_or_default().to_string(),
        })
        .collect(),
    }
  }
}

impl From<git2::Tag<'_>> for ObjectContents {
  fn from(tag: git2::Tag) -> Self {
    ObjectContents::Tag {
      name: tag.name().unwrap_or_default().to_string(),
      target: tag.target_id().to_string(),
    }
  }
}

impl From<git2::Blob<'_>> for ObjectContents {
  fn from(blob: git2::Blob) -> Self {
    ObjectContents::Blob {
      size: blob.content().len(),
    }
  }
}

#[derive(Serialize)]
struct Object {
  oid: String,
  contents: ObjectContents,
}

impl From<git2::Object<'_>> for Object {
  fn from(obj: git2::Object) -> Self {
    let oid = obj.id().to_string();
    let contents = match obj.kind() {
      Some(git2::ObjectType::Commit) => obj.into_commit().unwrap().into(),
      Some(git2::ObjectType::Tree) => obj.into_tree().unwrap().into(),
      Some(git2::ObjectType::Blob) => obj.into_blob().unwrap().into(),
      Some(git2::ObjectType::Tag) => obj.into_tag().unwrap().into(),
      Some(git2::ObjectType::Any) => ObjectContents::Unknown,
      None => ObjectContents::Unknown,
    };
    Object { oid, contents }
  }
}

#[derive(Serialize)]
struct Repo {
  index: Vec<String>,
  objects: Vec<String>,
}

fn git_data_raw(path: String) -> Result<Repo, git2::Error> {
  let repo = git2::Repository::open(&path)?;
  let idx = repo.index()?;
  let mut objects = Vec::with_capacity(256);
  repo.odb()?.foreach(|&oid| {
    objects.push(oid.to_string());
    true
  })?;
  Ok(Repo {
    index: idx.iter().map(|entry| entry.id.to_string()).collect(),
    objects,
  })
}

fn tostr<I: ToString>(e: I) -> String {
  e.to_string()
}

#[tauri::command]
fn git_data(path: String) -> Result<Repo, String> {
  git_data_raw(path).map_err(tostr)
}

#[tauri::command]
fn read_object(path: &str, id: String) -> Result<Object, String> {
  let repo = git2::Repository::open(&path).map_err(tostr)?;
  let oid :git2::Oid= id.parse().map_err(tostr)?;
  let git_obj = repo.find_object(oid, None).map_err(tostr)?;
  Ok(git_obj.into())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![git_data, read_object])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
