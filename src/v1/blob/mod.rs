use anyhow::Error;
mod download;
mod insert;
mod list;
mod properties;

pub use list::parse_list_body;
pub use list::EnumerationResults;

pub struct PropertiesResponse {
    pub last_modified: String,
}
pub struct Blob<'a> {
    account: &'a str,
    key: &'a str,
    container: &'a str,
    version_value: String,
}

impl<'a> Blob<'a> {
    pub fn new(account: &'a str, key: &'a str, container: &'a str) -> Self {
        Self {
            account,
            key,
            container,
            version_value: String::from("2015-02-21"),
        }
    }
    fn container_uri(&self) -> String {
        format!(
            "https://{}.blob.core.windows.net/{}",
            self.account, self.container
        )
    }
    // fn headers(&self) {}
    fn sign(
        &self,
        action: &Actions,
        file_name: &str,
        time_str: &str,
        content_length: usize,
    ) -> Result<String, Error> {
        let string_to_sign = prepare_to_sign(
            self.account,
            self.container,
            file_name,
            action,
            time_str,
            content_length,
            &self.version_value,
        );

        Ok(crate::sign::hmacsha256(self.key, &string_to_sign)?)
    }
}

enum Actions {
    Download,
    Insert,
    Properties,
    List,
}

impl From<&Actions> for http::Method {
    fn from(action: &Actions) -> Self {
        match action {
            Actions::Download => http::Method::GET,
            Actions::Insert => http::Method::PUT,
            Actions::Properties => http::Method::HEAD,
            Actions::List => http::Method::GET,
        }
    }
}

fn prepare_to_sign(
    account: &str,
    container: &str,
    obj: &str,
    action: &Actions,
    time_str: &str,
    content_length: usize,
    version_value: &str,
) -> String {
    {
        let content_encoding = "";
        let content_language = "";
        let content_length = {
            if content_length == 0 {
                String::from("")
            } else {
                content_length.to_string()
            }
        };
        let content_md5 = "";
        let content_type = "";
        let date = "";
        let if_modified_since = "";
        let if_match = "";
        let if_none_match = "";
        let if_unmodified_since = "";
        let range = "";
        let canonicalized_headers = format!(
            "x-ms-blob-type:{}\nx-ms-date:{}\nx-ms-version:{}",
            "BlockBlob", time_str, version_value
        );
        let verb = http::Method::from(action).to_string();
        let canonicalized_resource = match action {
            Actions::List => format!("/{}/{}\ncomp:list\nrestype:container", account, container),
            _ => format!("/{}/{}/{}", account, container, obj),
        };
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            verb,
            content_encoding,
            content_language,
            content_length,
            content_md5,
            content_type,
            date,
            if_modified_since,
            if_match,
            if_none_match,
            if_unmodified_since,
            range,
            canonicalized_headers,
            canonicalized_resource,
        )
    }
}
