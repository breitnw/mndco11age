pub enum HttpError<'a> {
    ClientError(&'a str),
    ServerError(&'a str),
}

impl<'a> HttpError<'a> {
    fn to_string(&self) -> String {
        match self {
            HttpError::ClientError(s) => {
                format!("client error: {s}")
            }
            HttpError::ServerError(s) => {
                format!("server error: {s}")
            }
        }
    }
    pub fn to_query(&self) -> String {
        let err_string = self.to_string();
        let e = percent_encoding::utf8_percent_encode(
            &err_string,
            percent_encoding::NON_ALPHANUMERIC,
        );
        format!("err={e}")
    }
}
