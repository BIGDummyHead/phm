use crate::HttpMethod;

pub struct HttpRequestMeta<'req> {
    route: &'req str,
    pub method: HttpMethod,
}

impl<'req> HttpRequestMeta<'req> {
    fn split_route(&self) -> (&'req str, Option<&'req str>) {
        let spl = self.dirty_route().rsplit_once("?");

        match spl {
            Some((a, b)) => (a, Some(b)),
            None => (self.dirty_route(), None),
        }
    }

    /// # Clean Route
    ///
    /// Returns the route slice as cleaned (meaning missing the ending parameters)
    ///
    /// `/api/user?name=Shawn` -> `/api/user`
    pub fn clean_route(&self) -> &'req str {
        self.split_route().0 //always choose 0
    }

    pub fn dirty_route(&self) -> &'req str {
        &self.route
    }
}
