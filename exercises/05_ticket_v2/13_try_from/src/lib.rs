// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

fn str2status(value: &str) -> Result<Status, &'static str> {
    let value = value.to_lowercase();
    match value.as_str() {
        "todo" => Ok(Status::ToDo),
        "inprogress" => Ok(Status::InProgress),
        "done" => Ok(Status::Done),
        _ => Err("Unknown Status"),
    }
}

impl TryFrom<String> for Status {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        str2status(&value)
    }
}

impl TryFrom<&str> for Status {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        str2status(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
