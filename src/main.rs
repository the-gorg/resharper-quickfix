use serde;
use serde::Deserialize;
use serde_xml_rs::{self, from_str};

fn main() {
    // Args in order: solution_path, file_path
    let mut args = std::env::args();

    let solution_path = args.nth(1).unwrap();
    let solution_name = solution_path.split("/").last().unwrap();

    let filename_path = args.nth(0).unwrap();
    let filename_name = filename_path.split("/").last().unwrap();

    println!("solution: {} filename: {}", solution_name, filename_name);

    std::fs::create_dir_all(format!(
        "~/.cache/resharper/{}/{}",
        solution_name, filename_name
    ))
    .expect("oh no");

    let command = std::process::Command::new("/home/gorg/bin/resharper/inspectcode.sh")
        .arg(&solution_path)
        .arg("--caches-home=/home/gorg/.cache/resharper/")
        .arg(format!("--include={}", filename_path))
        .arg("--no-swea")
        .arg(format!(
            "-o={}",
            format!(
                "/home/gorg/.cache/resharper/{}/{}.xml",
                solution_name, filename_name
            ),
        ))
        .arg("--f=Xml")
        .output()
        .expect("erppp");
    println!("{:?}", command);

    let file = std::fs::read_to_string(format!(
        "/home/gorg/.cache/resharper/{}/{}.xml",
        solution_name, filename_name
    ))
    .expect(
        &format!(
            "/home/gorg/.cache/resharper/{}/{}.xml",
            solution_name, filename_name
        )[..],
    );

    let test: Report = from_str(&file[..]).unwrap();
    println!("{:?}", test.issue_types);
    println!(
        "## Solution: {}",
        test.information.solution.replace("\\", "/")
    );
    for issue in test.issues[0].projects[0].list.iter() {
        println!(
            "{}:{}:0: >{}< {}",
            issue.file.replace("\\", "/"),
            issue.line,
            test.issue_types
                .types
                .iter()
                .filter(|it| it.id == issue.typeid)
                .last()
                .unwrap()
                .severity,
            issue.messsage
        );
    }
}

#[derive(Deserialize)]
struct Report {
    #[serde(rename = "Issues", default)]
    issues: Vec<Issues>,
    #[serde(rename = "Information")]
    information: Information,
    #[serde(rename = "IssueTypes")]
    issue_types: IssueTypes,
}

#[derive(Deserialize, Debug)]
struct IssueTypes {
    #[serde(rename = "IssueType", default)]
    types: Vec<IssueType>,
}

#[derive(Deserialize, Debug)]
struct IssueType {
    #[serde(rename = "Id", default)]
    id: String,
    #[serde(rename = "Severity", default)]
    severity: String,
}

#[derive(Deserialize)]
struct Information {
    #[serde(rename = "Solution")]
    solution: String,
}

#[derive(Deserialize)]
struct Issues {
    #[serde(rename = "Project", default)]
    projects: Vec<Project>,
}

#[derive(Deserialize)]
struct Project {
    #[serde(rename = "Issue", default)]
    list: Vec<Issue>,
}

#[derive(Deserialize, Debug)]
struct Issue {
    #[serde(rename = "TypeId", default)]
    typeid: String,
    #[serde(rename = "File", default)]
    file: String,
    #[serde(rename = "Line", default)]
    line: u16,
    #[serde(rename = "Message", default)]
    messsage: String,
}
