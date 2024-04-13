use std::cmp::Ordering;

use crate::find::projects::Finder;

fn last_match_percent(s: &str, rgx: &regex::Regex) -> f64 {
    let shortest_match = rgx.find_iter(s).last().map(|m| m.end()).unwrap_or(1);
    shortest_match as f64 / s.len() as f64
}

/// get_rightmost_match sorts the matched_projects by how far right the match is in the directory
/// path. For an example consider these matched projects for the term "suffix":
///
///   - /some/path/projecta-suffix
///   - /some/path/suffix-ex
///   - /some/path/project-suffix-in-the-middle
///
/// Each of these projects would be matched because they contain the word "suffix" however if we
/// want to narrow this down the "best" match we need to determine which of them matches suffix
/// closest to the end of its string.
///
/// We do this by calculating as a percentage how far right the end of the match is from the end of
/// the string. So in the case of `/some/path/projecta-suffix` it would be 100% as it's the end of
/// that string. In the case of `/some/path/suffix-ex` it would be 80% because the end of suffix is
/// the 16th character out of 20.
///
/// We then sort by highest percentage match and return the first result.
///
/// In the case of a tie for highest percentage match we prefer the shortest string. So if we added
/// the result `/some/path/pa-suffix` to the set above it would also have a 100% match but because
/// it is shorter than `/some/path/projecta-suffix` it is considered a better match and so will be
/// sorted higher.
fn get_rightmost_match<'a>(
    matched_projects: &'a mut Vec<String>,
    reverse: bool,
    rgx: &'a regex::Regex,
) -> Option<&'a String> {
    matched_projects.sort_by(|a, b| {
        let match_distance_a = last_match_percent(a, &rgx);
        let match_distance_b = last_match_percent(b, &rgx);
        let less_than = match_distance_a < match_distance_b;
        let equal = match_distance_a == match_distance_b;
        if less_than {
            Ordering::Less
        } else if equal {
            if a.len() == b.len() {
                Ordering::Equal
            } else if a.len() < b.len() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Greater
        }
    });

    if reverse {
        matched_projects.first()
    } else {
        matched_projects.last()
    }
}

pub fn find(finder: Finder, search_term: String, reverse: bool, verbose: bool) {
    let rgx = regex::Regex::new(&search_term).expect("Unable to compile regex!");

    let mut matched_projects = Vec::new();
    for project in finder {
        let project_path = project.as_os_str().to_string_lossy();
        if rgx.is_match(&project_path) {
            matched_projects.push(project_path.to_string().clone());
        }
    }

    if matched_projects.is_empty() {
        println!("No projects matched that search.");
        return;
    }

    if verbose {
        for project in matched_projects {
            println!("{}", project);
        }

        return;
    }

    let possible_match = get_rightmost_match(&mut matched_projects, reverse, &rgx);
    if let Some(found) = possible_match {
        println!("{}", found);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_last_match_percent() {
        let rgx = Regex::new("mat").unwrap();
        let path1 = last_match_percent("/home/mathewrobinson/Code/cdb", &rgx);
        let path2 = last_match_percent("/home/mathewrobinson/Code/mat", &rgx);
        assert!(path2 > path1);
    }

    #[test]
    fn test_last_match_percent_returns_percentage() {
        let rgx = Regex::new("mat").unwrap();
        let path1 = last_match_percent(
            "/Users/mathewrobinson/Work/applications/core_user_to_hubspot_client_match",
            &rgx,
        );
        let path2 = last_match_percent("/Users/mathewrobinson/Code/mat", &rgx);
        assert!(path2 > path1);
    }

    #[test]
    fn test_get_rightmost_match() {
        let mut projects = vec![
            "/Users/mathewrobinson/Work/applications/core_user_to_hubspot_client_match".to_string(),
            "/Users/mathewrobinson/Code/mat".to_string(),
            "/Users/mathewrobinson/Code/cdb".to_string(),
        ];
        let rgx = Regex::new("mat").unwrap();

        assert_eq!(
            get_rightmost_match(&mut projects, false, &rgx),
            Some(&"/Users/mathewrobinson/Code/mat".to_string())
        )
    }
}
