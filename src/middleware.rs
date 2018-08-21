use iron::prelude::*;
use iron::{status,AfterMiddleware};
use iron::headers::ContentType;
use router::NoRoute;

pub struct DefaultContentTypeMiddleware;
impl AfterMiddleware for DefaultContentTypeMiddleware {
    fn after(&self, _req: &mut Request, mut res: Response) -> IronResult<Response> {
        if res.headers.get::<ContentType>() == None {
            // Set a standard header
            res.headers.set(ContentType::html());
        }
        Ok(res)
    }
}

pub struct Custom404Middleware;
impl AfterMiddleware for Custom404Middleware {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if err.error.is::<NoRoute>() {
            Ok(Response::with((status::NotFound, "404: Not Found")))
        } else {
            Err(err)
        }
    }
}